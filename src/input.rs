use std::io::Write;

use crossterm::{cursor, terminal, QueueableCommand};

use crate::LeadrError;

pub fn clear_bottom_line() -> std::io::Result<()> {
    let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
    let (_cols, rows) = terminal::size().unwrap_or((80, 24));
    let y = rows.saturating_sub(1);

    stdout
        .queue(cursor::SavePosition)?
        .queue(cursor::MoveTo(0, y))?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
        .queue(cursor::RestorePosition)?
        .flush()?;

    Ok(())
}

pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> Result<Self, LeadrError> {
        terminal::enable_raw_mode().map_err(LeadrError::TerminalRawModeError)?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = clear_bottom_line();
        let _ = terminal::disable_raw_mode();
    }
}
