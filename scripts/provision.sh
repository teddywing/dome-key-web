#!/bin/sh

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

set -ex

OPT="$HOME/opt"


if [ ! -f "$OPT/bin/migrate" ]; then
	echo "Installing 'migrate'..."

	mkdir -p "$OPT/bin"

	curl \
		--remote-name \
		--location 'https://github.com/golang-migrate/migrate/releases/download/v4.0.2/migrate.linux-amd64.tar.gz'

	tar xf migrate.linux-amd64.tar.gz
	mv migrate.linux-amd64 "$OPT/bin/migrate"
	rm migrate.linux-amd64.tar.gz
fi
