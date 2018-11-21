// Copyright (c) 2018  Teddy Wing
//
// This file is part of DomeKey Web.
//
// DomeKey Web is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// DomeKey Web is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with DomeKey Web. If not, see
// <https://www.gnu.org/licenses/>.

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
{url}

Install DomeKey with:

	$ brew install teddywing/DomeKey/dome-key

Add your license by running:

	$ dome-key --license PATH/TO/dome-key-license.plist
",
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
