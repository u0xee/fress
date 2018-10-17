#!/usr/bin/env python3

import argparse
import os
import subprocess
import shutil
import sys


src_dir = 'src'
doc_dir = 'doc'
target_dir = 'target/doc'


def find_files(d, find_args=None):
    if not find_args:
        find_args = []
    command = ['find', '-L', d,
               '-type', 'f']
    command.extend(find_args)
    ret = subprocess.run(command, stdout=subprocess.PIPE).stdout.splitlines()
    return [str(f, 'utf-8') for f in ret]


def generate_html_from_asciidoc(dir, out_dir):
    adocs = find_files(dir, ['-name', '*.adoc'])
    command = ['asciidoctor', '--destination-dir', out_dir]
    command.extend(adocs)
    print('Running: {}'.format(' '.join(command)))
    subprocess.run(command)


def build_project(args):
    print('==== Creating rustdoc pages')
    subprocess.run(["cargo", "doc"])
    generate_html_from_asciidoc(doc_dir, target_dir)


# Main parser
parser = argparse.ArgumentParser(description='Builds AsciiDoc and rustdoc web pages.')
parser.set_defaults(func=lambda args: parser.print_usage())
subparsers = parser.add_subparsers(help='action to perform')

# Build subparser
build_parser = subparsers.add_parser('build', description='Build project docs',
                                     help='Build AsciiDoc and rustdoc web pages (-h for options)')
build_parser.set_defaults(func=build_project)


# Parse and dispatch
args = parser.parse_args()
args.func(args)
