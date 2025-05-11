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

#[derive(Debug)]
pub enum LeadrError {
    TerminalSetup(String),
    ReadError(String),
    InvalidKeymapError(String),
}
