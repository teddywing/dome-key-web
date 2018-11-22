dome-key-web
============

The [DomeKey][DomeKey] website. Mostly written in static HTML & CSS. License
generation is handled by a couple of FastCGI binaries.


## Setup

	$ ./bin/setup --install

This will install the required package dependencies with Homebrew and set up the
project.


## Running

	$ make run

Build the Rust binaries:

	$ cd license-generator/
	$ cargo build

CSS is built with [hasp][hasp]. After making a CSS change, run:

	$ make css


## Keys

A [Paddle][Paddle] public key must be added at:

	license-generator/private/paddle.pubkey.asc

[Aquatic Prime][Aquatic Prime] keys must be added as hex strings in:

	license-generator/private/public_key.txt
	license-generator/private/private_key.txt


## License
Copyright © 2018 Teddy Wing.

* [aquatic-prime][./aquatic-prime] is licensed under the GNU GPLv3+.
* [paddle][./paddle] is licensed under the GNU GPLv3+.
* Nearly all other source code is licensed under the GNU AGPLv3+.


[DomeKey]: https://domekey.teddywing.com/
[hasp]: https://github.com/djanowski/hasp
[Paddle]: https://paddle.com/
[Aquatic Prime]: https://github.com/bdrister/AquaticPrime/
[./aquatic-prime]: license-generator/aquatic-prime
[./paddle]: license-generator/paddle
