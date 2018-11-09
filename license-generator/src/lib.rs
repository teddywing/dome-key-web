#[macro_use]
extern crate error_chain;
extern crate mysql;
extern crate rand;
extern crate sha1;

mod errors {
    use mysql;

    error_chain! {
        foreign_links {
            MySql(mysql::error::Error);
        }
    }
}

use errors::*;

struct Purchaser<'a> {
    name: &'a str,
    email: &'a str,
    secret: Option<String>,
}

impl<'a> Purchaser<'a> {
    fn new(name: &'a str, email: &'a str) -> Self {
        Purchaser {
            name: name,
            email: email,
            secret: None,
        }
    }

    fn with_secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }

    fn generate_secret(&mut self) {
        let random: usize = rand::random();

        let source = format!("{}{}{}", self.name, self.email, random);
        let digest = sha1::Sha1::from(source).hexdigest();

        self.secret = Some(digest);
    }

    fn insert(&self, cx: &mut mysql::Conn) -> Result<()> {
        let mut tx = cx.start_transaction(
            false,  // consistent_snapshot
            None,   // isolation_level
            None,   // readonly
        )?;

        match self.secret {
            Some(ref s) => {
                tx.prep_exec("
                    INSERT INTO purchasers
                        (name, email, secret)
                        VALUES
                        (?, ?, ?)",
                    (self.name, self.email, s),
                )?;
            },
            None => {
                tx.prep_exec("
                    INSERT INTO purchasers
                        (name, email)
                        VALUES
                        (?, ?)",
                    (self.name, self.email),
                )?;
            },
        }

        Ok(())
    }
}
