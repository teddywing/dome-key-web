use base64;
use log;
use mysql;
use paddle;

error_chain! {
    foreign_links {
        EnvVar(::std::env::VarError);
        Io(::std::io::Error);

        Base64(base64::DecodeError);
        Log(log::SetLoggerError);
        MySql(mysql::error::Error);

        Paddle(paddle::errors::Error);
    }
}
