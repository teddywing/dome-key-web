///! FastCGI script that displays a thank-you page with a link to download a
///! custom-generated license.

extern crate fastcgi;

#[macro_use]
extern crate log;

extern crate license_generator;

use std::borrow::Cow;
use std::io::{Read, Write};

use license_generator::database;
use license_generator::errors::*;
use license_generator::logger;
use license_generator::params;
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

        // method = req.param("REQUEST_METHOD")
        //     .unwrap_or("REQUEST_METHOD".into()),
        // path = req.param("SCRIPT_NAME")
        //     .unwrap_or("SCRIPT_NAME".into()),
        // query = req.param("QUERY_STRING")
        //     .unwrap_or("QUERY_STRING".into()),

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
                "/license/download" => {
                    // Send Zip file
                    // method POST

                    let ps = params::parse(&params);
                    let name = ps.get("name");
                    let email = ps.get("email");
                    let secret = ps.get("secret");

                    if name.is_some() && email.is_some() && secret.is_some() {
                        let mut tx = cx.start_transaction(false, None, None).unwrap();
                        let query_result = tx.prep_exec("
                            SELECT id FROM purchasers
                            WHERE
                                name = ?
                            AND
                                email = ?
                            AND
                                secret = ?",
                            (
                                name.unwrap().to_string(),
                                email.unwrap().to_string(),
                                secret.unwrap().to_string(),
                            )
                        ).unwrap().next();

                        info!("Row: {:?}", query_result);

                        tx.commit().unwrap();
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
