#!/usr/bin/env python3

# Copyright (c) 2018  Teddy Wing
#
# This file is part of DomeKey Web.
#
# DomeKey Web is free software: you can redistribute it and/or modify it
# under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# DomeKey Web is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public
# License along with DomeKey Web. If not, see
# <https://www.gnu.org/licenses/>.

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
    logo = b64encode(f.read().encode('utf-8')).decode('utf-8')

template = Template(template)
html = template.substitute(
    CSS=css,
    LOGO_DATA=logo,
)

print(html, end='')
