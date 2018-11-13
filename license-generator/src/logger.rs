use std::env;
use std::fs::OpenOptions;

use fastcgi;
use simplelog::{Config, LevelFilter, WriteLogger};

use errors::*;

pub fn init() -> Result<()> {
    let log_file_path = env::var("LOG_FILE")
        .chain_err(|| "LOG_FILE environment variable not found")?;

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
        "{method} {path} {query} - {protocol} - {user_agent} - {remote_addr} | {forwarded_for} / {post_params}",
        method = req.param("REQUEST_METHOD")
            .unwrap_or("REQUEST_METHOD".into()),
        path = req.param("SCRIPT_NAME")
            .unwrap_or("SCRIPT_NAME".into()),
        query = req.param("QUERY_STRING")
            .unwrap_or("QUERY_STRING".into()),
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