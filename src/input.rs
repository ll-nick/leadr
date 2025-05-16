use crossterm::terminal;

use crate::LeadrError;

/// Guard that enables raw mode on creation and disables it on drop while also clearing the bottom line.
pub struct RawModeGuard;

impl RawModeGuard {
    /// Enables raw terminal mode.
    ///
    /// Used to capture keystrokes without requiring Enter.
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
