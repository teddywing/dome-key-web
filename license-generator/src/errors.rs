use log;
use mysql;

error_chain! {
    foreign_links {
        EnvVar(::std::env::VarError);
        Io(::std::io::Error);

        Log(log::SetLoggerError);
        MySql(mysql::error::Error);
    }
}
