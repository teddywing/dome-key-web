extern crate aquatic_prime;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::env;

use aquatic_prime::AquaticPrime;

#[derive(Serialize)]
struct LicenseData<'a> {
    #[serde(rename = "Name")]
    name: &'a str,

    #[serde(rename = "Email")]
    email: &'a str,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let aquatic_prime = AquaticPrime::new(&args[1], &args[2]);

    let license_data = LicenseData {
        name: &args[3],
        email: &args[4],
    };

    let plist = aquatic_prime.plist(license_data).unwrap();

    println!("{}", plist);
}
