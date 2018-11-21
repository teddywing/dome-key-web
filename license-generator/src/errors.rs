// Copyright (c) 2018  Teddy Wing
//
// This file is part of DomeKey Web.
//
// DomeKey Web is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// DomeKey Web is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with DomeKey Web. If not, see
// <https://www.gnu.org/licenses/>.

use aquatic_prime;
use log;
use mysql;
use paddle;
use url;
use zip_lib;

error_chain! {
    foreign_links {
        EnvVar(::std::env::VarError);
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);

        Log(log::SetLoggerError);
        MySql(mysql::error::Error);
        UrlParse(url::ParseError);
        Zip(zip_lib::result::ZipError);

        AquaticPrime(aquatic_prime::errors::Error);
        Paddle(paddle::errors::Error);
    }
}
