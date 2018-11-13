use std::io::Write;

use errors::*;

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
