use thiserror::Error;

#[derive(Debug, Error)]
pub enum LeadrError {
    #[error("Io error")]
    Io(#[from] std::io::Error),

    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Environment error")]
    Env(#[from] std::env::VarError),

    #[error("Conflicting sequence: {0}")]
    ConflictingSequenceError(String),

    #[error("Invalid keymap: {0}")]
    InvalidKeymapError(String),

    #[error("Invalid surround command: {0}")]
    InvalidSurroundCommand(String),
}
