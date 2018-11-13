use std::env;

use mysql;

use errors::*;

pub fn get_database_pool() -> Result<mysql::Pool> {
    let connection_url = env::var("DATABASE_URL")
        .chain_err(|| "DATABASE_URL environment variable not found")?;
    let pool = mysql::Pool::new_manual(4, 15, connection_url)?;

    Ok(pool)
}
