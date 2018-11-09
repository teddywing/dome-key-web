use mysql;

error_chain! {
    foreign_links {
        EnvVar(::std::env::VarError);

        MySql(mysql::error::Error);
    }
}
