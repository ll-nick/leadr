use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::collections::HashMap;

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
        enable_raw_mode().map_err(|e| LeadrError::TerminalSetup(e.to_string()))?;

        loop {
            if let Event::Key(KeyEvent { code, .. }) =
                read().map_err(|e| LeadrError::ReadError(e.to_string()))?
            {
                match code {
                    KeyCode::Char(c) => {
                        self.sequence.push(c);
                        if let Some(shortcut) = self.shortcuts.get(&self.sequence) {
                            if shortcut.execute {
                                disable_raw_mode()
                                    .map_err(|e| LeadrError::TerminalSetup(e.to_string()))?;
                                return Ok(ShortcutResult::Execute(shortcut.command.to_string()));
                            } else {
                                disable_raw_mode()
                                    .map_err(|e| LeadrError::TerminalSetup(e.to_string()))?;
                                return Ok(ShortcutResult::Insert(shortcut.command.to_string()));
                            }
                        }

                        let partial_match =
                            self.shortcuts.keys().any(|k| k.starts_with(&self.sequence));
                        if !partial_match {
                            disable_raw_mode()
                                .map_err(|e| LeadrError::TerminalSetup(e.to_string()))?;
                            return Ok(ShortcutResult::NoMatch);
                        }
                    }
                    KeyCode::Backspace => {
                        self.sequence.pop();
                    }
                    KeyCode::Esc => {
                        disable_raw_mode().map_err(|e| LeadrError::TerminalSetup(e.to_string()))?;
                        return Ok(ShortcutResult::Cancelled);
                    }
                    _ => {}
                }
            }
        }
    }
}
