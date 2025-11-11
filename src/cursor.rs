use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::time::Duration;

use mio::{Events, Interest, Poll, Token, unix::SourceFd};

use crate::LeadrError;

/// Query the terminal for the current cursor position (col, row)
///
/// Returns `(col, row)` as 0-based coordinates on success.
/// This cannot be done via crossterm because the shell integration pipes stdin/stdout,
/// so we have to open `/dev/tty` directly which is not currently supported by crossterm.
/// See https://github.com/crossterm-rs/crossterm/issues/919
pub fn query_cursor_position(timeout_ms: u64) -> Result<(u16, u16), LeadrError> {
    // Open the controlling terminal directly
    let mut tty = OpenOptions::new().read(true).write(true).open("/dev/tty")?;

    // Send the ANSI cursor position report query
    tty.write_all(b"\x1b[6n")?;
    tty.flush()?;

    // Prepare Mio polling
    let fd = tty.as_raw_fd();
    let mut poll = Poll::new()?;
    poll.registry()
        .register(&mut SourceFd(&fd), Token(0), Interest::READABLE)?;

    let mut events = Events::with_capacity(4);

    // Wait for up to `timeout_ms` for the terminal to respond
    poll.poll(&mut events, Some(Duration::from_millis(timeout_ms)))?;

    if events.is_empty() {
        return Err(LeadrError::TerminalQueryError(
            "Timed out waiting for cursor position response".into(),
        ));
    }

    // Read the response
    let mut response_buffer = [0u8; 64];
    let bytes_read = tty.read(&mut response_buffer)?;
    let response_string = String::from_utf8_lossy(&response_buffer[..bytes_read]);

    parse_cursor_response(&response_string)
}

/// Parse the terminal response with format ESC [ row ; col R
/// and return (col, row) as 0-based coordinates.
fn parse_cursor_response(s: &str) -> Result<(u16, u16), LeadrError> {
    let coords = s
        .strip_prefix("\x1b[")
        .ok_or_else(|| LeadrError::TerminalQueryError("Response missing ESC[".into()))?
        .strip_suffix('R')
        .ok_or_else(|| LeadrError::TerminalQueryError("Response missing trailing R".into()))?;

    let mut parts = coords.split(';');

    let row: u16 = parts
        .next()
        .ok_or_else(|| LeadrError::TerminalQueryError("Missing row value".into()))?
        .parse()?;

    let col: u16 = parts
        .next()
        .ok_or_else(|| LeadrError::TerminalQueryError("Missing column value".into()))?
        .parse()?;

    Ok((col - 1, row - 1)) // convert to 0-based
}
