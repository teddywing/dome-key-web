use log;
use mysql;
use paddle;

error_chain! {
    foreign_links {
        EnvVar(::std::env::VarError);
        Io(::std::io::Error);

        Log(log::SetLoggerError);
        MySql(mysql::error::Error);

        Paddle(paddle::errors::Error);
    }
}
