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

use std::io::Write;

use errors::*;

pub fn set_400<W: Write>(w: &mut W) -> Result<()> {
    Ok(writeln!(w, "Status: 400")?)
}

pub fn set_403<W: Write>(w: &mut W) -> Result<()> {
    Ok(writeln!(w, "Status: 403")?)
}

pub fn set_404<W: Write>(w: &mut W) -> Result<()> {
    Ok(writeln!(w, "Status: 404")?)
}

pub fn set_405<W: Write>(w: &mut W, allowed_methods: &str) -> Result<()> {
    Ok(
        writeln!(
            w,
            "Status: 405
Allow: {}",
            allowed_methods)?
    )
}

pub fn set_500<W: Write>(w: &mut W) -> Result<()> {
    Ok(writeln!(w, "Status: 500")?)
}

pub fn error_400<W: Write>(w: &mut W) {
    write!(w, "Status: 400
Content-Type: text/plain

400 Bad Request")
        .unwrap_or(());
}

pub fn error_403<W: Write>(w: &mut W, message: Option<&str>) {
    set_403(w).unwrap_or(());
    write!(
        w,
        "Content-Type: text/plain

403 Forbidden{}",
        message.map_or_else(
            || String::new(),
            |m| format!(": {}", m)
        )
    ).unwrap_or(());
}

pub fn error_404<W: Write>(w: &mut W) {
    write!(w, "Status: 404
Content-Type: text/plain

404 Not Found")
        .unwrap_or(());
}

pub fn error_405<W: Write>(w: &mut W, allowed_methods: &str) {
    set_405(w, allowed_methods).unwrap_or(());
    write!(w, "Content-Type: text/plain

405 Method Not Allowed")
        .unwrap_or(());
}

pub fn error_500<W: Write>(w: &mut W, error: Option<Error>) {
    if let Some(error) = error {
        error!("{}", error);
    }

    set_500(w).unwrap_or(());
    write!(w, "Content-Type: text/plain

500 Internal Server Error")
        .unwrap_or(());
}
