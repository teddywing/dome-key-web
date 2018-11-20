use mysql;

use errors::*;

pub fn get_database_pool() -> Result<mysql::Pool> {
    let connection_url = env!("DATABASE_URL");
    let pool = mysql::Pool::new_manual(4, 15, connection_url)?;

    Ok(pool)
}
