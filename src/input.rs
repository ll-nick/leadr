use crate::models::LeadrError;
use crossterm::terminal;

pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> Result<Self, LeadrError> {
        terminal::enable_raw_mode().map_err(LeadrError::TerminalRawModeError)?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}
