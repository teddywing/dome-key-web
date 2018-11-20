#!/bin/bash

set -ex


export PATH="$HOME/opt/bin:$PATH"

source "$HOME/production-config/env"

echo 'Moving files...'

mv "$REMOTE_PUBLIC_WWW/fulfillment" "$REMOTE_PUBLIC_WWW/fulfillment.fcgi"
mv "$REMOTE_PUBLIC_WWW/license" "$REMOTE_PUBLIC_WWW/license.fcgi"

mv production-config/.htaccess "$REMOTE_PUBLIC_WWW"


echo 'Provisioning...'

sh "$HOME/scripts/provision.sh"


echo 'Migrating database...'

migrate-up
