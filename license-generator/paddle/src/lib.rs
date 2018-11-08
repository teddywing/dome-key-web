extern crate openssl;

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Verifier;


// https://paddle.com/docs/reference-verifying-webhooks/
fn verify_signature<'a, I>(pem: &[u8], signature: &str, params: I) -> bool
where I: IntoIterator<Item = (&'a str, &'a str)> + PartialOrd {
    let rsa = Rsa::public_key_from_pem(pem).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha1(), &pkey).unwrap();
    verifier.update(signature.as_bytes()).unwrap();

    let signature = php_serialize(params);

    verifier.verify(signature.as_ref()).unwrap()
}

fn php_serialize<'a, I>(pairs: I) -> String
where I: IntoIterator<Item = (&'a str, &'a str)> + PartialOrd {
    let mut serialized = String::with_capacity(500);

    let mut len = 0;
    for (key, value) in pairs {
        serialized.push_str(
            &format!(
                "s:{key_length}:\"{key}\";s:{value_length}:\"{value}\";",
                key_length = key.len(),
                key = key,
                value_length = value.len(),
                value = value
            )
        );

        len += 1;
    }

    serialized.push_str("}");

    format!(
        "a:{pairs_count}:{{{rest}",
        pairs_count = len,
        rest = serialized
    )
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn php_serialize_serializes_key_values_as_php_string() {
        let mut data = BTreeMap::new();
        data.insert("checkout_id", "1234asdfjkl");
        data.insert("currency", "USD");
        data.insert("customer_name", "Senjougahara");

        let expected = r#"a:3:{s:11:"checkout_id";s:11:"1234asdfjkl";s:8:"currency";s:3:"USD";s:13:"customer_name";s:12:"Senjougahara";}"#;

        let serialized_data = php_serialize(data);

        assert_eq!(serialized_data, expected);
    }
}
