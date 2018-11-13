///! FastCGI script that displays a thank-you page with a link to download a
///! custom-generated license.

extern crate aquatic_prime;
extern crate fastcgi;
extern crate exitcode;

#[macro_use]
extern crate log;
extern crate mysql;

#[macro_use]
extern crate serde_derive;

extern crate license_generator;

use std::borrow::Cow;
use std::io::{Cursor, Read, Write};

use aquatic_prime::AquaticPrime;

use license_generator::database;
use license_generator::errors::*;
use license_generator::logger;
use license_generator::params;
use license_generator::response;
use license_generator::zip;

#[derive(Serialize)]
struct LicenseData<'a> {
    #[serde(rename = "Name")]
    name: &'a str,

    #[serde(rename = "Email")]
    email: &'a str,
}

fn query_purchaser(
    cx: &mut mysql::PooledConn,
    name: &str,
    email: &str,
    secret: &str,
) -> Result<Option<std::result::Result<mysql::Row, mysql::Error>>> {
    let mut tx = cx.start_transaction(false, None, None)?;
    let row = tx.prep_exec("
        SELECT id FROM purchasers
        WHERE
            name = ?
        AND
            email = ?
        AND
            secret = ?",
        (
            &name,
            &email,
            &secret,
        )
    )?.next();

    tx.commit()?;

    Ok(row)
}

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

    let public_key = include_str!("../../private/public_key.txt");
    let private_key = include_str!("../../private/private_key.txt");
    let aquatic_prime = AquaticPrime::new(&public_key, &private_key);

    fastcgi::run(move |mut req| {
        let mut params = String::new();
        match req.stdin().read_to_string(&mut params) {
            Ok(_) => (),
            Err(e) => error!("{}", e),
        }

        logger::log_request(&req, &params);

        let mut cx = match pool.get_conn() {
            Ok(cx) => cx,
            Err(e) => {
                return response::error_500(
                    &mut req.stdout(),
                    Some(e.into())
                );
            },
        };

        if let Some(path) = req.param("REQUEST_URI") {
            match path.as_ref() {
                "/license" => {
                    // Get params name, email, secret
                    // Render thank-you page with link to download file
                },

                // Respond with a zip archive of the license file
                "/license/download" => {
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
                    let name = ps.get("name");
                    let email = ps.get("email");
                    let secret = ps.get("secret");

                    if name.is_some() && email.is_some() && secret.is_some() {
                        let name = name.unwrap().to_string();
                        let email = email.unwrap().to_string();
                        let secret = secret.unwrap().to_string();

                        let purchaser = match query_purchaser(&mut cx, &name, &email, &secret) {
                            Ok(p) => p,
                            Err(e) => return response::error_500(
                                &mut req.stdout(),
                                Some(e.into())
                            ),
                        };

                        if let Some(purchaser) = purchaser {
                            match purchaser {
                                Ok(p) => p,
                                Err(e) => return response::error_500(
                                    &mut req.stdout(),
                                    Some(e.into())
                                ),
                            };

                            let license_data = LicenseData {
                                name: &name,
                                email: &email,
                            };

                            let license = match aquatic_prime.plist(license_data) {
                                Ok(p) => p,
                                Err(e) => return response::error_500(
                                    &mut req.stdout(),
                                    Some(e.into())
                                ),
                            };

                            let mut zip_data = Cursor::new(vec![]);
                            match zip::license(&mut zip_data, license.as_bytes()) {
                                Ok(p) => p,
                                Err(e) => return response::error_500(
                                    &mut req.stdout(),
                                    Some(e.into())
                                ),
                            }

                            write!(&mut req.stdout(), "Content-Type: application/zip
Content-Disposition: attachment; filename=\"dome-key-license.zip\"\n\n")
                                .and_then(|_|
                                    req.stdout().write_all(&zip_data.into_inner())
                                ).unwrap_or(());
                        }
                    } else {
                        error!(
                            "Missing request parameters: name: '{}', email: '{}', secret: '{}'",
                            name.unwrap_or(&Cow::Borrowed("")),
                            email.unwrap_or(&Cow::Borrowed("")),
                            secret.unwrap_or(&Cow::Borrowed("")),
                        );
                    }
                },
                _ => (),
            }
        }
    });

    Ok(())
}
