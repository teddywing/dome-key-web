.PHONY: css
css: assets/styles.css

assets/styles.css: assets/stylesheets/main.hcss \
		assets/stylesheets/*.hcss \
		assets/stylesheets/*.css
	hasp $< > $@
	sed -i .bak '/^$$/d' $@
	rm "$@.bak"

internal_error.html: internal_error.in.html
	./scripts/generate_500.py > $@
