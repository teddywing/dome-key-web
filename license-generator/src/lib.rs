#[macro_use]
extern crate error_chain;
extern crate mysql;

mod errors {
    error_chain! {}
}

use errors::*;

struct Purchaser<'a> {
    name: &'a str,
    email: &'a str,
    secret: Option<&'a str>,
}

impl<'a> Purchaser<'a> {
    fn new(name: &'a str, email: &'a str) -> Self {
        Purchaser {
            name: name,
            email: email,
            secret: None,
        }
    }

    fn with_secret(mut self, secret: &'a str) -> Self {
        self.secret = Some(secret);
        self
    }

    fn insert() -> Result<()> {
        unimplemented!()
    }
}
