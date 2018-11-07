extern crate base64;

#[macro_use]
extern crate error_chain;
extern crate openssl;
extern crate plist;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod errors {
    use plist;

    error_chain! {
        foreign_links {
            FromUtf8(::std::string::FromUtf8Error);

            Plist(plist::Error);
        }

        errors {
            PublicKeyIncorrectNumBits(bits: i32) {
                description("public key has incorrect bit size")
                display("public key has incorrect bit size: '{}'", bits)
            }

            InvalidLicenseData {
                display("license data must be a dictionary")
            }
        }
    }
}

use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;

use openssl::bn::BigNum;
use openssl::rsa::Padding;
use openssl::rsa::RsaPrivateKeyBuilder;
use openssl::sha::sha1;
use plist::Plist;
use serde::ser::Serialize;

use errors::*;

struct AquaticPrime<'a> {
    public_key: &'a str,
    private_key: &'a str,
}

impl<'a> AquaticPrime<'a> {
    fn sign(&self, input_data: HashMap<String, String>) -> Result<[u8; 128]> {
        let mut input_data: Vec<(String, String)> = input_data
            .into_iter()
            .collect();
        input_data.sort_unstable_by_key(|el| el.0.to_lowercase());

        let data = input_data
            .into_iter()
            .map(|(_k, v)| v)
            .collect::<Vec<String>>()
            .concat();

        let public_key = self.public_key.trim_left_matches("0x");
        let private_key = self.private_key.trim_left_matches("0x");

        let public_key = BigNum::from_hex_str(public_key)
            .chain_err(|| "public key could not be converted to BigNum")?;
        let private_key = BigNum::from_hex_str(private_key)
            .chain_err(|| "private key could not be converted to BigNum")?;
        let rsa_e = BigNum::from_u32(3)
            .chain_err(|| "public exponent could not be converted to BigNum")?;

        let public_key_bit_size = public_key.num_bits();
        if public_key_bit_size != 1024 {
            return Err(
                ErrorKind::PublicKeyIncorrectNumBits(public_key_bit_size).into()
            );
        }

        let digest = sha1(data.as_bytes());

        let keypair = RsaPrivateKeyBuilder::new(
            public_key,
            rsa_e,
            private_key,
        )
            .chain_err(|| "failed to build RSA key")?
            .build();

        let mut signature = [0; 128];
        keypair.private_encrypt(
            &digest,
            &mut signature,
            Padding::PKCS1,
        ).chain_err(|| "failed to encrypt input")?;

        Ok(signature)
    }

    fn plist<T: Serialize>(&self, input_data: T) -> Result<String> {
        // Get input as `Plist`
        let mut xml_for_plist = Vec::with_capacity(600);

        plist::serde::serialize_to_xml(&mut xml_for_plist, &input_data)?;
        let xml_for_hash_map = xml_for_plist.clone();

        let plist_data = Plist::read(Cursor::new(&xml_for_plist))?;

        let mut plist_dict = plist_data
            .as_dictionary()
            .ok_or(ErrorKind::InvalidLicenseData)?
            .to_owned();

        // Get input as HashMap to send to `sign()`
        let data: HashMap<String, String> = plist::serde::deserialize(
            Cursor::new(&xml_for_hash_map)
        )?;

        let signature = self.sign(data)?;
        plist_dict.insert(
            "Signature".to_owned(),
            Plist::Data(signature.to_vec())
        );

        // Generate plist XML string
        let mut plist_xml = Cursor::new(Vec::with_capacity(600));

        {
            let mut writer = plist::xml::EventWriter::new(&mut plist_xml);

            for item in Plist::Dictionary(plist_dict).into_events() {
                writer.write(&item)?;
            }
        }

        Ok(String::from_utf8(plist_xml.into_inner())?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_KEY: &'static str = "0xAAD0DC5705017D4AA1CD3FA194771E97B263E68\
        308DC09D3D9297247D175CCD05DFE410B9426D3C8019BA6B92D34F21B454D8D8AC8CAD2\
        FB37850987C02592012D658911442C27F4D9B050CFA3F7C07FF81CFEEBE33E1E43595B2\
        ACCC2019DC7247829017A91D40020F9D8BF67300CE744263B4F34FF42E3A7BE3CF37C40\
        04EB";
    const PRIVATE_KEY: &'static str = "0x71E092E4AE00FE31C1337FC10DA4BF0FCC4299\
        ACB092B137E61BA185364E888AE9542B5D0D6F37DAABBD19D0C8CDF6BCD8DE5E5C85DC8\
        CA77A58B1052AC3B6AA5C7EA2E58BD484050184D2E241CFCB1D6AB4AC86174990560608\
        33D8F6699B9C54E3BAA36123AFD5B4DDE6F2ADFC08F6970C3BA5C80B9A0A04CB6C6B73D\
        D512B";

    fn initialize_aquatic_prime<'a>() -> AquaticPrime<'a> {
        AquaticPrime {
            public_key: PUBLIC_KEY,
            private_key: PRIVATE_KEY,
        }
    }

    #[test]
    fn sign_produces_a_correct_signature() {
        let aquatic_prime = initialize_aquatic_prime();

        let mut license_data = HashMap::new();
        license_data.insert("Email".to_owned(), "user@email.com".to_owned());
        license_data.insert("Name".to_owned(), "User".to_owned());

        let signature = aquatic_prime.sign(license_data);

        let expected = "Nhe6U/8XCMm7/+2OIzrHjcOsYHNZTg4k8nTajp1dTb+pU5H1cybgQzUJYA1n3IIQAbWe\
            qD7a48WFqbzC3powTk6x42b+WpH6boe+u7LW4AXo2ZqGPasVlr1/lUWVHvt5J0OI9oR7\
            vmzdXHbbQD7RPXp0ezttrKBFHxNNCbJHMr0=";

        assert_eq!(base64::encode(&signature.unwrap()[..]), expected);


        let mut license_data = HashMap::new();
        license_data.insert("Email".to_owned(), "user@email.com".to_owned());
        license_data.insert("Name".to_owned(), "Üsér Diacriticà".to_owned());
        license_data.insert(
            "lowercase key".to_owned(),
            "Keys should be sorted case-insensitive".to_owned()
        );

        let signature = aquatic_prime.sign(license_data);

        let expected = "RIhF/3CgyXzPg2wCQ5LShf6W9khtqPcqUDLAHcAZdOIcoeR7PoOHi15423kxq5jOh1lm\
            cztBoUJFu8mB45MHE0jmmbRw3qK6FJz9Py2gi1XvGOgH3GW713OCvQBE7vfBj4ZriP0+\
            FS18nLfrtM6Xp0mAd1la4DD4oh7d35dlYTY=";

        assert_eq!(base64::encode(&signature.unwrap()[..]), expected);
    }

    #[test]
    fn plist_produces_a_license_plist_string() {
        let aquatic_prime = initialize_aquatic_prime();

        let mut license_data = HashMap::new();
        license_data.insert("Email", "user@email.com");
        license_data.insert("Name", "Üsér Diacriticà");
        license_data.insert("lowercase key", "Keys should be sorted case-insensitive");

        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>Email</key>
	<string>user@email.com</string>
	<key>Name</key>
	<string>Üsér Diacriticà</string>
	<key>Signature</key>
	<data>RIhF/3CgyXzPg2wCQ5LShf6W9khtqPcqUDLAHcAZdOIcoeR7PoOHi15423kxq5jOh1lmcztBoUJFu8mB45MHE0jmmbRw3qK6FJz9Py2gi1XvGOgH3GW713OCvQBE7vfBj4ZriP0+FS18nLfrtM6Xp0mAd1la4DD4oh7d35dlYTY=</data>
	<key>lowercase key</key>
	<string>Keys should be sorted case-insensitive</string>
</dict>
</plist>"#;

        assert_eq!(
            aquatic_prime.plist(license_data).unwrap(),
            expected
        );
    }

    #[test]
    fn plist_takes_a_generic_struct() {
        let aquatic_prime = initialize_aquatic_prime();

        #[derive(Serialize)]
        struct LicenseData<'a> {
            #[serde(rename = "Name")]
            name: &'a str,

            #[serde(rename = "Email")]
            email: &'a str,
        };

        let license_data = LicenseData {
            name: "User",
            email: "user@example.com",
        };

        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>Email</key>
	<string>user@example.com</string>
	<key>Name</key>
	<string>User</string>
	<key>Signature</key>
	<data>djFsFgYkt/ESgOnR+dpOFBTqpgWCG9aZggOYG/zv3uOEMf39Zwt5m7L+ulFjmZvfvUR/twuwKRfPWLGjoDHPQqwbBED3PcIP4asBeRbt28y6425tah4KV5SVnzVmZgAwCjkeuOEO5WPljiPXkvbUEVqNaEm79moMkHO9nYKdnP0=</data>
</dict>
</plist>"#;

        assert_eq!(
            aquatic_prime.plist(license_data).unwrap(),
            expected
        );
    }
}
