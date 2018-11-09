use std::env;

use mysql;

use errors::*;

fn get_database_connection() -> Result<mysql::PooledConn> {
    let connection_url = env::var("DATABASE_URL")?;
    let pool = mysql::Pool::new_manual(10, 50, connection_url)?;
    let cx = pool.get_conn()?;

    Ok(cx)
}
