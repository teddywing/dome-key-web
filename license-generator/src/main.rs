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
use license_generator::purchaser::Purchaser;
use license_generator::request;

fn main() -> Result<()> {
    let log_file_path = env::var("LOG_FILE")
        .chain_err(|| "LOG_FILE environment variable not found")?;

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)?;

    let mut log_config = Config::default();
    log_config.time_format = Some("%+");
    WriteLogger::init(LevelFilter::Info, log_config, log_file)?;

    let mut cx = match database::get_database_connection()
        .chain_err(|| "failed to create a database connection")
    {
        Ok(cx) => cx,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        },
    };

    let test_purchaser = Purchaser::new("Shiki", "shiki@example.com");
    match test_purchaser.insert(&mut cx) {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }

    fastcgi::run(|mut req| {
        write!(&mut req.stdout(), "Content-Type: text/plain\n\nHello, world!")
            .unwrap_or(());

        let mut params = String::new();
        for (key, val) in req.params() {
            params.push_str(format!("{}: {}\n", key, val).as_str());
        }

        info!("{}", params);

        let mut stdin = String::new();
        req.stdin().read_to_string(&mut stdin).unwrap();

        info!("{}", stdin);

        let is_verified = request::verified(&stdin);
        info!("{:?}", is_verified);
    });

    Ok(())
}
