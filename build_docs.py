#!/usr/bin/env python3

import argparse
import os
import subprocess

src_dir = 'src'
doc_dir = 'doc'
target_dir = 'target/doc'

subprocess.run(['mkdir', '--parents', target_dir])


def find_files(d, find_args=None):
    if not find_args:
        find_args = []
    command = ['find', '-L', d] + find_args + ['-type', 'f']
    ret = subprocess.run(command, stdout=subprocess.PIPE).stdout.splitlines()
    return [str(f, 'utf-8') for f in ret]


asciidoc_base = ['asciidoctor',
                 '-a', 'doctype=article',
                 '-a', 'sectanchors',
                 '-a', 'imagesdir=images',
                 '-a', 'stylesheet=style.css',
                 '-a', 'docinfo=shared',
                 '-a', 'idprefix=+',
                 '-a', 'idseparator=-',
                 '-a', 'toc=left']


reset, bold, underline, invert = '\u001b[0m', '\u001b[1m', '\u001b[4m', '\u001b[7m'


def is_byte(i):
    return 0 <= i <= 256


def color(byte):
    assert(is_byte(byte))
    # f'\u001b[48;5;{byte}m' background
    return f'\u001b[38;5;{byte}m'


def rgb(r, g, b):
    assert(is_byte(r) and is_byte(g) and is_byte(b))
    return f'\u001b[38;2;{r};{g};{b}m'


def run(command, env=None):
    program = f'⬛ {bold}{color(208)}{command[0]}{reset} '
    print(program + ' '.join(command[1:]))
    subprocess.run(command, env=env)


def generate_html_from_asciidoc(dir, out_dir):
    command = asciidoc_base + ['--destination-dir', out_dir]
    adocs = find_files(dir, ['-maxdepth', '1', '-name', '*.adoc'])
    full_command = command + adocs
    run(full_command)
    thesis_html = find_files(out_dir, ['-name', 'thesis.html'])
    subprocess.run(['sed', '-i', 's/100%px/100%/g'] + thesis_html)

    home = find_files(dir, ['-name', 'home.adoc'])
    subprocess.run(command + home)


def copy_images():
    run(['rsync', '-r', doc_dir + '/images', target_dir])


def copy_favicons():
    run(['rsync', '-r', doc_dir + '/images/favicon/', target_dir])


def build_adoc(args):
    generate_html_from_asciidoc(doc_dir, target_dir)
    copy_images()
    copy_favicons()


def after_cargo():
    print(f'{invert}{bold}                        cargo done                     {reset}')


def build_project(args):
    subprocess.run(["cargo", "doc"])
    after_cargo()
    build_adoc(args)


wasm_triple = 'wasm32-unknown-unknown'


def build_wasm_repl(args):
    e = os.environ.copy()
    e['RUSTFLAGS'] = '-C link-arg=--export-table'
    run(['cargo', 'build', '--target', wasm_triple], env=e)
    after_cargo()

    run(['rsync', 'target/' + wasm_triple + '/debug/fress.wasm', target_dir])
    js = find_files(src_dir, ['-name', '*.js'])
    run(['rsync'] + js + [target_dir])
    build_adoc(args)


def http_server(args):
    import http.server
    import socketserver
    import webbrowser
    os.chdir(target_dir)
    handler = http.server.SimpleHTTPRequestHandler
    handler.extensions_map = {'.manifest': 'text/cache-manifest',
                              '.html': 'text/html',
                              '.png': 'image/png',
                              '.jpg': 'image/jpg',
                              '.svg': 'image/svg+xml',
                              '.css': 'text/css',
                              '.js': 'application/x-javascript',
                              '.wasm': 'application/wasm',
                              '': 'application/octet-stream'}
    httpd = socketserver.TCPServer(("", args.port), handler)
    location = 'http://localhost:' + str(args.port) + '/thesis.html'
    print("Serving directory {} at port {}".format(target_dir, args.port))
    webbrowser.open(location)
    httpd.serve_forever()


# Main parser
parser = argparse.ArgumentParser(description='Builds AsciiDoc and rustdoc web pages.')
parser.set_defaults(func=build_project)
subparsers = parser.add_subparsers(help='action to perform')

# Build subparser
build_parser = subparsers.add_parser('build', description='Build project docs',
                                     help='Build AsciiDoc and rustdoc web pages (-h for options)')
build_parser.set_defaults(func=build_project)

# AsciiDoc subparser
adoc_parser = subparsers.add_parser('adoc', description='Build project asciidocs',
                                     help='Build AsciiDoc web pages (-h for options)')
adoc_parser.set_defaults(func=build_adoc)

# WASM REPL subparser
wasm_parser = subparsers.add_parser('wasm', description='Build wasm repl',
                                    help='Build wasm library (-h for options)')
wasm_parser.set_defaults(func=build_wasm_repl)

# Local server
server_parser = subparsers.add_parser('http', description='Start local file server.',
                                    help='Starts an http server, on port')
server_parser.set_defaults(func=http_server)
server_parser.add_argument('--port', default=8888, type=int)

# Parse and dispatch
args = parser.parse_args()
args.func(args)

# top priority todos
#homepage
#https://doc.rust-lang.org/rustdoc/command-line-arguments.html#--html-in-header-include-more-html-in-head
#https://blog.guillaume-gomez.fr/articles/2016-09-16+Generating+doc+with+rustdoc+and+a+custom+theme
