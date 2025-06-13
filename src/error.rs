use thiserror::Error;

#[derive(Debug, Error)]
pub enum LeadrError {
    #[error("Failed to determine config directory")]
    ConfigDirNotFound,

    #[error("Environment error")]
    Env(#[from] std::env::VarError),

    #[error("Io error")]
    Io(#[from] std::io::Error),

    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Failed to parse TOML: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("Failed to serialize TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Conflicting sequence: {0}")]
    ConflictingSequenceError(String),

    #[error("Invalid keymap: {0}")]
    InvalidKeymapError(String),

    #[error("Invalid surround command: {0}")]
    InvalidSurroundCommand(String),
}
