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
    use super::*;

    #[test]
    fn php_serialize_serializes_key_values_as_php_string() {
    }
}
