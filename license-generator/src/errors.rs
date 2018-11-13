use aquatic_prime;
use log;
use mysql;
use paddle;
use zip_lib;

error_chain! {
    foreign_links {
        EnvVar(::std::env::VarError);
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);

        Log(log::SetLoggerError);
        MySql(mysql::error::Error);
        Zip(zip_lib::result::ZipError);

        AquaticPrime(aquatic_prime::errors::Error);
        Paddle(paddle::errors::Error);
    }
}
