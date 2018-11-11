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
use license_generator::params;
use license_generator::purchaser::Purchaser;
use license_generator::request;
use license_generator::response;

fn log_request(req: &fastcgi::Request, post_params: &str) {
    info!(
        "{method} {path} {query} - {protocol} - {user_agent} - {remote_addr} | {forwarded_for} / {post_params}",
        method = req.param("REQUEST_METHOD")
            .unwrap_or("REQUEST_METHOD".into()),
        path = req.param("SCRIPT_NAME")
            .unwrap_or("SCRIPT_NAME".into()),
        query = req.param("QUERY_STRING")
            .unwrap_or("QUERY_STRING".into()),
        protocol = req.param("SERVER_PROTOCOL")
            .unwrap_or("SERVER_PROTOCOL".into()),
        user_agent = req.param("HTTP_USER_AGENT")
            .unwrap_or("HTTP_USER_AGENT".into()),
        remote_addr = req.param("REMOTE_ADDR")
            .unwrap_or("REMOTE_ADDR".into()),
        forwarded_for = req.param("HTTP_X_FORWARDED_FOR")
            .unwrap_or("HTTP_X_FORWARDED_FOR".into()),
        post_params = post_params,
    );
}

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

    let pool = match database::get_database_pool()
        .chain_err(|| "failed to create a database connection pool")
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        },
    };

    fastcgi::run(move |mut req| {
        let mut params = String::new();
        match req.stdin().read_to_string(&mut params) {
            Ok(_) => (),
            Err(e) => error!("{}", e),
        }

        log_request(&req, &params);

        match req.param("REQUEST_METHOD") {
            Some(method) => {
                if method != "POST" {
                    response::set_405(&mut req.stdout(), "POST")
                        .unwrap_or(());
                    write!(&mut req.stdout(), "Content-Type: text/plain

405 Method Not Allowed")
                        .unwrap_or(());

                    return;
                }
            },
            None => {
                return response::error_500(&mut req.stdout(), None);
            },
        };

        let ps = params::parse(&params);
        let is_verified = match request::verified(&ps) {
            Ok(v) => v,
            Err(e) => {
                return response::error_500(&mut req.stdout(), Some(e));
            },
        };

        if is_verified {
            let name = ps.get("name");
            let email = ps.get("email");

            if name.is_some() && email.is_some() {
                let purchaser = Purchaser::new(name.unwrap(), email.unwrap());

                let mut cx = match pool.get_conn() {
                    Ok(cx) => cx,
                    Err(e) => {
                        return response::error_500(
                            &mut req.stdout(),
                            Some(e.into())
                        );
                    },
                };

                match purchaser.insert(&mut cx) {
                    Ok(_) => {
                        // TODO: Print message to be appended to user email

                        write!(&mut req.stdout(), "Content-Type: text/plain

200 OK")
                            .unwrap_or(());

                        return;
                    },
                    Err(e) => {
                        return response::error_500(&mut req.stdout(), Some(e));
                    },
                }
            }

            return response::error_500(
                &mut req.stdout(),
                Some("Purchaser name or email not set".into())
            );
        }

        response::set_403(&mut req.stdout()).unwrap_or(());
        write!(&mut req.stdout(), "Content-Type: text/plain

403 Forbidden: Invalid request signature")
            .unwrap_or(());
    });

    Ok(())
}
