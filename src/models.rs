#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Shortcut {
    pub sequence: String,
    pub command: String,
    pub description: Option<String>,
    pub execute: bool,
}

pub enum ShortcutResult {
    Execute(String),
    Insert(String),
    Cancelled,
    NoMatch,
}

pub enum LeadrError {
    TerminalSetup(String),
    ReadError(String),
}

