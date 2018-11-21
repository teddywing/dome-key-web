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

use std::fs::OpenOptions;

use fastcgi;
use simplelog::{Config, LevelFilter, WriteLogger};

use errors::*;

pub fn init() -> Result<()> {
    let log_file_path = env!("LOG_FILE");

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)?;

    let mut log_config = Config::default();
    log_config.time_format = Some("%+");
    WriteLogger::init(LevelFilter::Info, log_config, log_file)?;

    Ok(())
}

pub fn log_request(req: &fastcgi::Request, post_params: &str) {
    info!(
        "{method} {path} - {protocol} - {user_agent} - {remote_addr} | {forwarded_for} / {post_params}",
        method = req.param("REQUEST_METHOD")
            .unwrap_or("REQUEST_METHOD".into()),
        path = req.param("REQUEST_URI")
            .unwrap_or("REQUEST_URI".into()),
        protocol = req.param("SERVER_PROTOCOL")
            .unwrap_or("SERVER_PROTOCOL".into()),
        user_agent = req.param("HTTP_USER_AGENT")
            .unwrap_or("HTTP_USER_AGENT".into()),
        remote_addr = req.param("REMOTE_ADDR")
            .unwrap_or("REMOTE_ADDR".into()),
        forwarded_for = req.param("HTTP_X_FORWARDED_FOR")
            .unwrap_or("HTTP_X_FORWARDED_FOR".into()),
        post_params = post_params,
    );
}
