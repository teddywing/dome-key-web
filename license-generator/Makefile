.PHONY: run
run:
	ls target/debug/fulfillment target/debug/license | \
		entr -r lighttpd -f lighttpd.conf -D
