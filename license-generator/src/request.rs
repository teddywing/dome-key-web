use paddle;

use errors::*;
use params;

pub fn verified(req_params: &str) -> Result<bool> {
    let p = params::parse(&req_params);
    let pem = include_bytes!("../private/paddle.pubkey.asc");

    Ok(paddle::verify_signature(pem, p)?)
}
