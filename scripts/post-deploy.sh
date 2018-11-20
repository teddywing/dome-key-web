#!/bin/bash

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
