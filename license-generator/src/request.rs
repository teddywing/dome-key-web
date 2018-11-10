use paddle;

use errors::*;
use params;

pub fn verified(req_params: &str) -> Result<bool> {
    let mut p = params::parse(&req_params);
    let signature = p.remove("p_signature");
    let pem = include_bytes!("../private/paddle.pubkey.asc");

    match signature {
        Some(signature) => Ok(paddle::verify_signature(pem, &signature, p)?),
        None => Ok(false),
    }
}