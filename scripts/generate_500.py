#!/usr/bin/env python3

from base64 import b64encode
from string import Template
import os


script_dir = os.path.dirname(__file__)

template = ''
css = ''
logo = ''

with open(os.path.join(script_dir, '../internal_error.in.html'), 'r') as template:
    template = template.read()

with open(os.path.join(script_dir, '../assets/styles.css'), 'r') as f:
    css = f.read()

with open(os.path.join(script_dir, '../assets/logo.svg'), 'r') as f:
    logo = b64encode(f.read().encode('utf-8'))

template = Template(template)
html = template.substitute(
    CSS=css,
    LOGO_DATA=logo,
)

print(html, end='')
