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

const PUBLIC_KEY: &'static str = include_str!("../../private/public_key.txt");
const PRIVATE_KEY: &'static str = include_str!("../../private/private_key.txt");

#[derive(Serialize)]
struct LicenseData<'a> {
    #[serde(rename = "Name")]
    name: &'a str,

    #[serde(rename = "Email")]
    email: &'a str,
}

trait LicenseValidationResponse {
    fn success(&mut self, name: &str, email: &str, secret: &str);
    fn error_400(&mut self);
    fn error_404(&mut self);
    fn error_500(&mut self, error: Option<Error>);
}

struct HtmlResponse<'a, W: 'a> {
    writer: &'a mut W,
}

impl<'a, W> LicenseValidationResponse for HtmlResponse<'a, W>
where W: 'a + Write {
    fn success(&mut self, name: &str, email: &str, secret: &str) {
        write!(
            self.writer,
            "Status: 200
Content-Type: text/html\n\n{}",
            format!(
                include_str!("../../../thank-you.html"),
                name = name,
                email = email,
                secret = secret,
            )
        ).unwrap_or(())
    }

    fn error_400(&mut self) {
        let page_400 = include_str!("../../../400.html");
        response::set_400(self.writer)
            .and_then(|_|
                Ok(write!(self.writer, "Content-Type: text/html\n\n{}", page_400)?)
            ).unwrap_or(())
    }

    fn error_404(&mut self) {
        let page_404 = include_str!("../../../404.html");
        response::set_404(self.writer)
            .and_then(|_|
                Ok(write!(self.writer, "Content-Type: text/html\n\n{}", page_404)?)
            ).unwrap_or(())
    }

    fn error_500(&mut self, error: Option<Error>) {
    }
}

struct ZipResponse<'a, W: 'a> {
    writer: &'a mut W,
}

impl<'a, W> LicenseValidationResponse for ZipResponse<'a, W>
where W: 'a + Write {
    fn success(&mut self, name: &str, email: &str, _secret: &str) {
        let license_data = LicenseData {
            name: &name,
            email: &email,
        };

        let aquatic_prime = AquaticPrime::new(&PUBLIC_KEY, &PRIVATE_KEY);
        let license = match aquatic_prime.plist(license_data) {
            Ok(p) => p,
            Err(e) => return self.error_500(Some(e.into())),
        };

        let mut zip_data = Cursor::new(vec![]);
        match zip::license(&mut zip_data, license.as_bytes()) {
            Ok(p) => p,
            Err(e) => return self.error_500(Some(e.into())),
        }

        write!(self.writer, "Content-Type: application/zip
Content-Disposition: attachment; filename=\"dome-key-license.zip\"\n\n")
            .and_then(|_|
                self.writer.write_all(&zip_data.into_inner())
            ).unwrap_or(());
    }

    fn error_400(&mut self) {
        response::error_400(self.writer);
    }

    fn error_404(&mut self) {
        response::error_404(self.writer);
    }

    fn error_500(&mut self, error: Option<Error>) {
        response::error_500(self.writer, error)
    }
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

fn build_response<'a, R: LicenseValidationResponse>(
    cx: &mut mysql::PooledConn,
    params: &str,
    responses: &mut R,
) {
    let params = params::parse(&params);
    let name = params.get("name");
    let email = params.get("email");
    let secret = params.get("secret");

    if name.is_some() && email.is_some() && secret.is_some() {
        let name = name.unwrap().to_string();
        let email = email.unwrap().to_string();
        let secret = secret.unwrap().to_string();

        let purchaser = match query_purchaser(cx, &name, &email, &secret) {
            Ok(p) => p,
            Err(e) => return responses.error_500(Some(e.into())),
        };

        if let Some(purchaser) = purchaser {
            match purchaser {
                Ok(p) => p,
                Err(e) => return responses.error_500(Some(e.into())),
            };

            return responses.success(&name, &email, &secret);
        } else {
            return responses.error_404();
        }
    } else {
        error!(
            "Missing request parameters: name: '{}', email: '{}', secret: '{}'",
            name.unwrap_or(&Cow::Borrowed("")),
            email.unwrap_or(&Cow::Borrowed("")),
            secret.unwrap_or(&Cow::Borrowed("")),
        );

        return responses.error_400();
    }
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
            let path = path.split("?").collect::<Vec<_>>()[0];

            match path.as_ref() {
                "/license" => {
                    // Get params name, email, secret
                    // Render thank-you page with link to download file
                    let params = req.param("QUERY_STRING").unwrap();
                    let mut responses = HtmlResponse { writer: &mut req.stdout() };
                    return build_response(&mut cx, &params, &mut responses);

                    // TODO: Extract from /license/download
                    // trait for HTML vs. text/zip
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

                    let mut responses = ZipResponse { writer: &mut req.stdout() };
                    return build_response(&mut cx, &params, &mut responses);
                },
                _ => (),
            }
        }
    });

    Ok(())
}
