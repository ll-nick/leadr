use thiserror::Error;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Shortcut {
    pub sequence: String,
    pub command: String,
    pub description: Option<String>,
    pub execute: bool,
}

impl Shortcut {
    pub fn format_command(&self, exec_prefix: &str) -> String {
        if self.execute {
            format!("{} {}", exec_prefix, self.command)
        } else {
            self.command.to_string()
        }
    }
}

pub enum ShortcutResult {
    Shortcut(Shortcut),
    Cancelled,
    NoMatch,
}

#[derive(Debug, Error)]
pub enum LeadrError {
    #[error("Conflicting sequence: {0}")]
    ConflictingSequenceError(String),

    #[error("Failed to read environment variable: {0}")]
    EnvVarReadError(#[source] std::env::VarError),

    #[error("Failed to read user input: {0}")]
    InputReadError(#[source] std::io::Error),

    #[error("Invalid keymap: {0}")]
    InvalidKeymapError(String),

    #[error("Invalid surround command: {0}")]
    InvalidSurroundCommand(String),

    #[error("Failed parsing: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Failed to enable terminal raw mode: {0}")]
    TerminalRawModeError(#[source] std::io::Error),

    #[error("Failed to operate on tty: {0}")]
    TtyError(#[source] std::io::Error),
}
