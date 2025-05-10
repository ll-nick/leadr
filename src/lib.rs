use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::collections::HashMap;

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

    pub fn run(&mut self) -> Result<String, String> {
        enable_raw_mode().expect("Failed to enter raw mode");

        loop {
            if let Event::Key(KeyEvent { code, .. }) = read().expect("Failed to read event") {
                match code {
                    KeyCode::Char(c) => {
                        self.sequence.push(c);
                        if let Some(shortcut) = self.shortcuts.get(&self.sequence as &str) {
                            if shortcut.execute {
                                disable_raw_mode().expect("Failed to disable raw mode");
                                return Ok(format!("#EXEC {}", shortcut.command));
                            } else {
                                disable_raw_mode().expect("Failed to disable raw mode");
                                return Ok(shortcut.command.to_string());
                            }
                        }

                        let partial_match =
                            self.shortcuts.keys().any(|k| k.starts_with(&self.sequence));
                        if !partial_match {
                            disable_raw_mode().expect("Failed to disable raw mode");
                            return Err(format!("No match for sequence: {}", self.sequence));
                        }
                    }
                    KeyCode::Backspace => {
                        self.sequence.pop();
                    }
                    KeyCode::Esc => {
                        disable_raw_mode().expect("Failed to disable raw mode");
                        return Err("Escape pressed".to_string());
                    }
                    _ => {}
                }
            }
        }
    }
}
