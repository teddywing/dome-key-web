use std::env;

use mysql;

use errors::*;

pub fn get_database_connection() -> Result<mysql::PooledConn> {
    let connection_url = env::var("DATABASE_URL")
        .chain_err(|| "DATABASE_URL environment variable not found")?;
    let pool = mysql::Pool::new_manual(10, 50, connection_url)?;
    let cx = pool.get_conn()?;

    Ok(cx)
}
