extern crate fastcgi;

#[macro_use]
extern crate log;
extern crate mysql;
extern crate simplelog;

extern crate license_generator;

use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use simplelog::{Config, LevelFilter, WriteLogger};

use license_generator::database;
use license_generator::errors::*;

fn main() -> Result<()> {
    let log_file_path = env::var("LOG_FILE")?;

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)?;

    WriteLogger::init(LevelFilter::Info, Config::default(), log_file)?;

    let cx = match database::get_database_connection()
        .chain_err(|| "failed to create a database connection")
    {
        Ok(cx) => cx,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        },
    };

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

    Ok(())
}
