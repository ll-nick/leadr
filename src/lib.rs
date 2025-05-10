use crossterm::event::{read, Event, KeyCode, KeyEvent};
use std::collections::HashMap;

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> Result<Self, String> {
        crossterm::terminal::enable_raw_mode()
            .map_err(|e| format!("Failed to enable raw mode: {}", e))?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}

pub enum LeadrError {
    TerminalSetup(String),
    ReadError(String),
}

pub enum ShortcutResult {
    Execute(String),
    Insert(String),
    Cancelled,
    NoMatch,
}

pub struct Shortcut {
    pub sequence: String,
    pub command: String,
    pub description: Option<String>,
    pub execute: bool,
}

pub struct ShortcutManager {
    shortcuts: HashMap<String, Shortcut>,
    sequence: String,
}

impl ShortcutManager {
    pub fn new(shortcuts: Vec<Shortcut>) -> Self {
        let mut map = HashMap::new();
        for shortcut in shortcuts {
            map.insert(shortcut.sequence.clone(), shortcut);
        }
        ShortcutManager {
            shortcuts: map,
            sequence: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<ShortcutResult, LeadrError> {
        let _guard = RawModeGuard::new().map_err(LeadrError::TerminalSetup)?;

        loop {
            if let Event::Key(KeyEvent { code, .. }) =
                read().map_err(|e| LeadrError::ReadError(e.to_string()))?
            {
                match code {
                    KeyCode::Char(c) => {
                        self.sequence.push(c);
                        if let Some(shortcut) = self.match_sequence(&self.sequence) {
                            if shortcut.execute {
                                return Ok(ShortcutResult::Execute(shortcut.command.to_string()));
                            } else {
                                return Ok(ShortcutResult::Insert(shortcut.command.to_string()));
                            }
                        }

                        if !self.has_partial_match(&self.sequence) {
                            return Ok(ShortcutResult::NoMatch);
                        }
                    }
                    KeyCode::Backspace => {
                        self.sequence.pop();
                    }
                    KeyCode::Esc => {
                        return Ok(ShortcutResult::Cancelled);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn match_sequence(&self, seq: &str) -> Option<&Shortcut> {
        self.shortcuts.get(seq)
    }

    pub fn has_partial_match(&self, seq: &str) -> bool {
        self.shortcuts.keys().any(|k| k.starts_with(seq))
    }
}
