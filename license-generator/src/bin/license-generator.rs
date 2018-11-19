extern crate fastcgi;

#[macro_use]
extern crate log;
extern crate mysql;
extern crate simplelog;
extern crate url;

extern crate license_generator;

use std::io::{Read, Write};

use url::Url;

use license_generator::database;
use license_generator::errors::*;
use license_generator::logger;
use license_generator::params;
use license_generator::purchaser::Purchaser;
use license_generator::request;
use license_generator::response;

fn main() -> Result<()> {
    logger::init()?;

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

        logger::log_request(&req, &params);

        match req.param("REQUEST_METHOD") {
            Some(method) => {
                if method != "POST" {
                    return response::error_405(&mut req.stdout(), "POST");
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

                        let secret = match purchaser.secret {
                            Some(s) => s,
                            None => return response::error_500(
                                &mut req.stdout(),
                                Some("Empty secret".into())
                            ),
                        };

                        let license_download_url = match Url::parse_with_params(
                            "https://domekey.teddywing.com/license",
                            &[
                                ("name", purchaser.name),
                                ("email", purchaser.email),
                                ("secret", &secret),
                            ],
                        ) {
                            Ok(u) => u,
                            Err(e) => return response::error_500(
                                &mut req.stdout(),
                                Some(e.into())
                            ),
                        };

                        write!(
                            &mut req.stdout(),
                            "Content-Type: text/plain

Thanks so much for purchasing DomeKey!

Download your license here:
{url}",
                            url = license_download_url,
                        )
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

        response::error_403(
            &mut req.stdout(),
            Some("Invalid request signature")
        );
    });

    Ok(())
}
