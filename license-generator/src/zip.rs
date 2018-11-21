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

use std::io::{Seek, Write};

use zip_lib as zip;

use errors::*;

pub fn license<W: Write + Seek>(w: &mut W, plist: &[u8]) -> Result<()> {
    let mut zip = zip::ZipWriter::new(w);

    zip.start_file(
        "dome-key-license.plist",
        zip::write::FileOptions::default()
    )?;
    zip.write(plist)?;
    zip.finish()?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::license;

    #[test]
    #[ignore]
    fn license_produces_a_valid_zip_file() {
        let mut file = File::create("/tmp/license.zip").unwrap();
        let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
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

        license(&mut file, plist.as_bytes()).unwrap();
    }
}
