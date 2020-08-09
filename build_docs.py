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


def generate_html_from_asciidoc(dir, out_dir):
    command = asciidoc_base + ['--destination-dir', out_dir]
    adocs = find_files(dir, ['-maxdepth', '1', '-name', '*.adoc'])
    full_command = command + adocs
    print('Running: {}'.format(' '.join(full_command)))
    subprocess.run(full_command)
    thesis_html = find_files(out_dir, ['-name', 'thesis.html'])
    subprocess.run(['sed', '-i', 's/100%px/100%/g'] + thesis_html)

    home = find_files(dir, ['-name', 'home.adoc'])
    subprocess.run(command + home)


def copy_images():
    command = ['rsync', '-r', doc_dir + '/images', target_dir]
    print('Running: {}'.format(' '.join(command)))
    subprocess.run(command)


def copy_favicons():
    command = ['rsync', '-r', doc_dir + '/images/favicon/', target_dir]
    print('Running: {}'.format(' '.join(command)))
    subprocess.run(command)


def build_adoc(args):
    generate_html_from_asciidoc(doc_dir, target_dir)
    copy_images()
    copy_favicons()


def build_project(args):
    subprocess.run(["cargo", "doc"])
    build_adoc(args)


wasm_triple = 'wasm32-unknown-unknown'


def build_wasm_repl(args):
    command = ['cargo', 'build', '--target', wasm_triple]
    print('Running: {}'.format(' '.join(command)))
    e = os.environ.copy()
    e['RUSTFLAGS'] = '-C link-arg=--export-table'
    subprocess.run(command, env=e)

    c = ['rsync', 'target/' + wasm_triple + '/debug/fress.wasm', target_dir]
    print('Running: {}'.format(' '.join(c)))
    subprocess.run(c)
    js = find_files(src_dir, ['-name', '*.js'])
    c = ['rsync'] + js + [target_dir]
    print('Running: {}'.format(' '.join(c)))
    subprocess.run(c)
    build_adoc(args)


def http_server(args):
    import http.server
    import socketserver
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
    port = 8888
    httpd = socketserver.TCPServer(("", port), handler)
    print("Serving directory {} at port {}".format(target_dir, port))
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

# Parse and dispatch
args = parser.parse_args()
args.func(args)

# top priority todos
#homepage
#https://doc.rust-lang.org/rustdoc/command-line-arguments.html#--html-in-header-include-more-html-in-head
#https://blog.guillaume-gomez.fr/articles/2016-09-16+Generating+doc+with+rustdoc+and+a+custom+theme
