extern crate openssl;

use std::collections::HashMap;

struct AquaticPrime<'a> {
    public_key: &'a str,
    private_key: &'a str,
}

impl<'a> AquaticPrime<'a> {
    fn sign(&self, input_data: &HashMap<&str, &str>) -> String {
        String::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_produces_a_correct_signature() {
        let public_key = "0xAAD0DC5705017D4AA1CD3FA194771E97B263E68308DC09D3D9297247D175CCD05DFE410B9426D3C8019BA6B92D34F21B454D8D8AC8CAD2FB37850987C02592012D658911442C27F4D9B050CFA3F7C07FF81CFEEBE33E1E43595B2ACCC2019DC7247829017A91D40020F9D8BF67300CE744263B4F34FF42E3A7BE3CF37C4004EB";
        let private_key = "0x71E092E4AE00FE31C1337FC10DA4BF0FCC4299ACB092B137E61BA185364E888AE9542B5D0D6F37DAABBD19D0C8CDF6BCD8DE5E5C85DC8CA77A58B1052AC3B6AA5C7EA2E58BD484050184D2E241CFCB1D6AB4AC8617499056060833D8F6699B9C54E3BAA36123AFD5B4DDE6F2ADFC08F6970C3BA5C80B9A0A04CB6C6B73DD512B";

        let aquatic_prime = AquaticPrime {
            public_key: public_key,
            private_key: private_key,
        };

        let mut license_data = HashMap::new();
        license_data.insert("Email", "user@email.com");
        license_data.insert("Name", "User");

        let signature = aquatic_prime.sign(&license_data);

        let expected = "Nhe6U/8XCMm7/+2OIzrHjcOsYHNZTg4k8nTajp1dTb+pU5H1cybgQzUJYA1n3IIQAbWe \
            qD7a48WFqbzC3powTk6x42b+WpH6boe+u7LW4AXo2ZqGPasVlr1/lUWVHvt5J0OI9oR7 \
            vmzdXHbbQD7RPXp0ezttrKBFHxNNCbJHMr0=";

        assert_eq!(signature, expected);


        let mut license_data = HashMap::new();
        license_data.insert("Email", "user@email.com");
        license_data.insert("Name", "Üsér Diacriticà");
        license_data.insert("lowercase key", "Keys should be sorted case-insensitive");

        let signature = aquatic_prime.sign(&license_data);

        let expected = "RIhF/3CgyXzPg2wCQ5LShf6W9khtqPcqUDLAHcAZdOIcoeR7PoOHi15423kxq5jOh1lm \
            cztBoUJFu8mB45MHE0jmmbRw3qK6FJz9Py2gi1XvGOgH3GW713OCvQBE7vfBj4ZriP0+ \
            FS18nLfrtM6Xp0mAd1la4DD4oh7d35dlYTY=";

        assert_eq!(signature, expected);
    }
}
