use std::borrow::Cow;
use std::collections::BTreeMap;

use paddle;

use errors::*;

pub fn verified<'a>(
    req_params: &BTreeMap<Cow<'a, str>, Cow<'a, str>>
) -> Result<bool> {
    let pem = include_bytes!("../private/paddle.pubkey.asc");

    Ok(paddle::verify_signature(pem, req_params.clone())?)
}
