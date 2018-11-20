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

# Compile binaries for Linux
license-generator/target/release/fulfillment \
license-generator/target/release/license: production-config/env
	docker run \
		--rm \
		--interactive \
		--tty \
		--volume $$PWD/license-generator:/app \
		--workdir /app \
		rust:1.30.1-trusty \
		bash -c " \
			source ../production-config/env; \
			cargo build --release \
		"

.PHONY: deploy
deploy: license-generator/target/release/fulfillment \
		license-generator/target/release/license \
		internal_error.html \
		assets/styles.css
	bash ./scripts/deploy.sh
