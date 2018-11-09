#[macro_use]
extern crate error_chain;
extern crate mysql;

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

    fn insert(&self, cx: &mut mysql::Conn) -> Result<()> {
        let mut tx = cx.start_transaction(
            false,  // consistent_snapshot
            None,   // isolation_level
            None,   // readonly
        )?;

        tx.prep_exec("
            INSERT INTO purchasers
                (name, email, secret)
                VALUES
                (?, ?, ?)",
            (self.name, self.email, self.secret),
        )?;

        Ok(())
    }
}
