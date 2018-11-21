// Copyright (c) 2018  Teddy Wing
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate base64;

#[macro_use]
extern crate error_chain;
extern crate openssl;

pub mod errors {
    use base64;
    use openssl;

    error_chain! {
        foreign_links {
            Base64(base64::DecodeError);
            Openssl(openssl::error::ErrorStack);
        }

        errors {
            SignatureNotFound {
                display("no signature could be found in params")
            }
        }
    }
}

use std::fmt::Display;
use std::ops::Deref;

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::sign::Verifier;

use errors::*;


// https://paddle.com/docs/reference-verifying-webhooks/
pub fn verify_signature<'a, S, I>(
    pem: &[u8],
    params: I,
) -> Result<bool>
where
    S: AsRef<str> + Deref<Target = str> + PartialEq<str> + PartialOrd + Display,
    I: IntoIterator<Item = (S, S)> + PartialOrd,
{
    let rsa = Rsa::public_key_from_pem(pem)?;
    let pkey = PKey::from_rsa(rsa)?;
    let mut verifier = Verifier::new(MessageDigest::sha1(), &pkey)?;

    let (signature_params, params): (Vec<_>, Vec<_>) = params
        .into_iter()
        .partition(|(k, _v)| k == "p_signature");
    let signature = &signature_params
        .first()
        .ok_or(ErrorKind::SignatureNotFound)?
        .1;
    let signature = base64::decode(signature.as_bytes())?;

    let digest = php_serialize(params);
    verifier.update(digest.as_bytes())?;

    Ok(verifier.verify(&signature)?)
}

fn php_serialize<'a, S, I>(pairs: I) -> String
where
    S: AsRef<str> + Deref<Target = str> + Display,
    I: IntoIterator<Item = (S, S)> + PartialOrd,
{
    let mut serialized = String::with_capacity(500);

    let mut len = 0;
    for (key, value) in pairs {
        serialized.push_str(
            &format!(
                "s:{key_length}:\"{key}\";s:{value_length}:\"{value}\";",
                key_length = key.chars().count(),
                key = key,
                value_length = value.chars().count(),
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
