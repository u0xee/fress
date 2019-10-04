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
    adocs = find_files(dir, ['-name', 'thesis.adoc'])
    # adocs = find_files(dir, ['-name', '*.adoc'])
    command = ['asciidoctor',
               '--destination-dir', out_dir,
               '-a', 'doctype=article',
               '-a', 'stylesheet=style.css',
               '-a', 'stylesdir=anc',
               '-a', 'imagesdir=images',
               '-a', 'docinfo=shared',
               '-a', 'docinfodir=images/favicon',
               '-a', 'icons=font',
               '-a', 'toc=left',
               '-a', 'source-highlighter=prettify']
    command.extend(adocs)
    print('Running: {}'.format(' '.join(command)))
    subprocess.run(command)


def copy_images():
    command = ['cp', '--recursive', '--update',
               doc_dir + '/images', target_dir]
    print('Running: {}'.format(' '.join(command)))
    subprocess.run(command)


def copy_favicons():
    command = ['cp', '--update']
    #command.append(doc_dir + '/images/favicon/favicon-32x32.png')
    command.extend(find_files(doc_dir + '/images/favicon'))
    command.append(target_dir)
    print('Running: {}'.format(' '.join(command)))
    subprocess.run(command)


def build_project(args):
    subprocess.run(["cargo", "doc"])
    generate_html_from_asciidoc(doc_dir, target_dir)
    copy_images()
    copy_favicons()

def build_adoc(args):
    generate_html_from_asciidoc(doc_dir, target_dir)
    copy_images()
    copy_favicons()

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


# Parse and dispatch
args = parser.parse_args()
args.func(args)

# 100.20.252.216
# http, html,
