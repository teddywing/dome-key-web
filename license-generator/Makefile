.PHONY: run
run:
	ls target/debug/license-generator target/debug/license | \
		entr -r lighttpd -f lighttpd.conf -D
