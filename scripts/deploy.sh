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


source license-generator/.env

echo 'Deploying...'

rsync -avz \
	--exclude 'assets/stylesheets' \
	.htaccess \
	400.html \
	404.html \
	assets \
	doc \
	downloads \
	index.html \
	internal_error.html \
	license-generator/target/release/fulfillment \
	license-generator/target/release/license \
	robots.txt \
	thank-you.html \
	"$SSH_SERVER":"$REMOTE_PUBLIC_WWW/"

rsync -avz \
	--exclude '.git' \
	license-generator/migrations \
	production-config \
	scripts \
	"$SSH_SERVER":~/


echo 'Running post-deploy script...'
ssh "$SSH_SERVER" 'bash ~/scripts/post-deploy.sh'
