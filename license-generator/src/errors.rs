use mysql;

error_chain! {
    foreign_links {
        MySql(mysql::error::Error);
    }
}
