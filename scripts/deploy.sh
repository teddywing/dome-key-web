#!/bin/bash

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
