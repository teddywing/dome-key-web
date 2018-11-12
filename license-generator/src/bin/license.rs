///! FastCGI script that displays a thank-you page with a link to download a
///! custom-generated license.

extern crate fastcgi;

#[macro_use]
extern crate log;

extern crate license_generator;

use std::io::{Read, Write};

use license_generator::errors::*;
use license_generator::logger;

fn main() -> Result<()> {
    logger::init()?;

    // let pool = match database::get_database_pool()
    //     .chain_err(|| "failed to create a database connection pool")
    // {
    //     Ok(pool) => pool,
    //     Err(e) => {
    //         error!("{}", e);
    //         return Err(e);
    //     },
    // };

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

        if let Some(path) = req.param("REQUEST_URI") {
            match path.as_ref() {
                "/license" => {
                    // Get params name, email, secret
                    // Render thank-you page with link to download file
                },
                "/license/download" => {
                    // Send Zip file
                    // method POST
                },
                _ => (),
            }
        }
    });

    Ok(())
}
