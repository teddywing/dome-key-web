use paddle;

use params;

pub fn verified(req_params: &str) -> bool {
    let mut p = params::parse(&req_params);
    let signature = p.remove("p_signature");
    let pem = include_bytes!("../private/paddle.pubkey.asc");

    match signature {
        Some(signature) => paddle::verify_signature(pem, &signature, p),
        None => false,
    }
}
