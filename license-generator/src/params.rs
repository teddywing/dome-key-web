// Copyright (c) 2018  Teddy Wing
//
// This file is part of DomeKey Web.
//
// DomeKey Web is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// DomeKey Web is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with DomeKey Web. If not, see
// <https://www.gnu.org/licenses/>.

use std::borrow::Cow;
use std::collections::BTreeMap;

use url::form_urlencoded;

pub fn parse<'a>(params: &'a str) -> BTreeMap<Cow<'a, str>, Cow<'a, str>> {
    let iter = form_urlencoded::parse(params.as_bytes());
    let mut dict = BTreeMap::new();

    for (key, value) in iter {
        dict.insert(key, value);
    }

    dict
}
