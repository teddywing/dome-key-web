use std::collections::BTreeMap;

use url::form_urlencoded;

pub fn parse(params: &str) -> BTreeMap<String, String> {
    let iter = form_urlencoded::parse(params.as_bytes()).into_owned();
    let mut dict = BTreeMap::new();

    for (key, value) in iter {
        dict.insert(key, value);
    }

    dict
}
