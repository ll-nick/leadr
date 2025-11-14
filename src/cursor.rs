use std::fs::OpenOptions;
use std::io::{Read, Write};

use color_eyre::eyre::{OptionExt, Result, eyre};

/// Query the terminal for the current cursor position (col, row)
///
/// Returns `(col, row)` as 0-based coordinates on success.
/// This cannot be done via crossterm because the shell integration redirects stdin/stdout,
/// so we have to open `/dev/tty` directly which is not currently supported by crossterm.
/// See https://github.com/crossterm-rs/crossterm/issues/919
pub fn query_cursor_position() -> Result<(u16, u16)> {
    // Open the controlling terminal directly
    let mut tty = OpenOptions::new().read(true).write(true).open("/dev/tty")?;

    // Send the ANSI cursor position report query
    tty.write_all(b"\x1b[6n")?;
    tty.flush()?;

    // Read the response
    let mut response_buffer = [0u8; 64];
    let bytes_read = tty.read(&mut response_buffer)?;
    let response_string = std::str::from_utf8(&response_buffer[..bytes_read])?.to_string();

    parse_cursor_response(&response_string)
}

/// Parse the terminal response with format ESC [ row ; col R
/// and return (col, row) as 0-based coordinates.
fn parse_cursor_response(s: &str) -> Result<(u16, u16)> {
    let coords = s
        .strip_prefix("\x1b[")
        .ok_or_else(|| eyre!("Terminal query response missing ESC[: {:?}", s))?
        .strip_suffix('R')
        .ok_or_else(|| eyre!("Terminal query response missing trailing R: {:?}", s))?;

    let mut parts = coords.split(';');

    let row: u16 = parts.next().ok_or_eyre("Missing row value.")?.parse()?;
    let col: u16 = parts.next().ok_or_eyre("Missing column value.")?.parse()?;

    Ok((col - 1, row - 1)) // convert to 0-based
}
