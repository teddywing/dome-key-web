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

extern crate aquatic_prime;
extern crate chrono;

#[macro_use]
extern crate error_chain;
extern crate fastcgi;

#[macro_use]
extern crate log;
extern crate mysql;
extern crate paddle;
extern crate rand;
extern crate simplelog;
extern crate sha1;
extern crate url;
extern crate zip as zip_lib;


pub mod database;
pub mod errors;
pub mod logger;
pub mod params;
pub mod purchaser;
pub mod request;
pub mod response;
pub mod zip;
