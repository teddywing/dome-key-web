extern crate fastcgi;

use std::io::{Read, Write};

fn main() {
    fastcgi::run(|mut req| {
        write!(&mut req.stdout(), "Content-Type: text/plain\n\nHello, world!")
            .unwrap_or(());

        let mut params = String::new();
        for (key, val) in req.params() {
            params.push_str(format!("{}: {}\n", key, val).as_str());
        }

        write!(&mut req.stdout(), "\n\n{}", params)
            .unwrap_or(());

        let mut stdin = String::new();
        req.stdin().read_to_string(&mut stdin).unwrap();

        write!(&mut req.stdout(), "\n\nstdin: {}\n", stdin)
            .unwrap_or(());
    });
}
