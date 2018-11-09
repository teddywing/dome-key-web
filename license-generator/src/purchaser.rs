use mysql;
use rand::{self, Rng};
use sha1;

use errors::*;

struct Purchaser<'a> {
    name: &'a str,
    email: &'a str,
    secret: Option<String>,
}

impl<'a> Purchaser<'a> {
    fn new(name: &'a str, email: &'a str) -> Self {
        let mut purchaser = Purchaser {
            name: name,
            email: email,
            secret: None,
        };

        purchaser.generate_secret();

        purchaser
    }

    fn generate_secret(&mut self) {
        let mut rng = rand::thread_rng();
        let random: usize = rng.gen_range(1_000_000_000, ::std::usize::MAX);

        let source = format!("{}{}{}", self.name, self.email, random);
        let digest = sha1::Sha1::from(source).hexdigest();

        self.secret = Some(digest);
    }

    fn insert(&self, cx: &mut mysql::PooledConn) -> Result<()> {
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
