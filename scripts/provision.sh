#!/bin/sh

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
