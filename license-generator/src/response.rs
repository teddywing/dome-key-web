use std::io::Write;

use errors::*;

pub fn set_403<W: Write>(w: &mut W) -> Result<()> {
    Ok(writeln!(w, "Status: 403")?)
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
