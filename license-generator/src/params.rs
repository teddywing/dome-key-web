use std::borrow::Cow;
use std::collections::BTreeMap;

use url::form_urlencoded;

pub(crate) fn parse<'a>(params: &'a str) -> BTreeMap<Cow<'a, str>, Cow<'a, str>> {
    let iter = form_urlencoded::parse(params.as_bytes());
    let mut dict = BTreeMap::new();

    for (key, value) in iter {
        dict.insert(key, value);
    }

    dict
}
