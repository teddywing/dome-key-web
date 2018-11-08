// https://paddle.com/docs/reference-verifying-webhooks/
fn verify_signature<'a, I>(params: I) -> bool
where I: ExactSizeIterator<Item = (&'a str, &'a str)> {
    false
}

fn php_serialize<'a, I>(pairs: I) -> String
where I: ExactSizeIterator<Item = (&'a str, &'a str)> {
    let mut serialized = String::with_capacity(500);

    serialized.push_str(
        &format!("a:{pairs_count}:{{", pairs_count = pairs.len())
    );

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
    }

    serialized.push_str("}");

    serialized
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
