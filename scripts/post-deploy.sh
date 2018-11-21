#!/bin/bash

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


export PATH="$HOME/opt/bin:$PATH"

source "$HOME/production-config/env"

echo 'Moving files...'

mv "$REMOTE_PUBLIC_WWW/fulfillment" "$REMOTE_PUBLIC_WWW/fulfillment.fcgi"
mv "$REMOTE_PUBLIC_WWW/license" "$REMOTE_PUBLIC_WWW/license.fcgi"


echo 'Restarting FastCGI scripts...'
set +e
killall fulfillment.fcgi
killall license.fcgi
set -e


echo 'Provisioning...'

sh "$HOME/scripts/provision.sh"


echo 'Migrating database...'

migrate-up
