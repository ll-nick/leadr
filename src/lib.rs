use crossterm::event::{read, Event, KeyCode, KeyEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub shortcuts: Vec<Shortcut>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            shortcuts: vec![
                // File navigation
                Shortcut {
                    sequence: "ll".into(),
                    command: "ls -la".into(),
                    description: Some("List directory contents (detailed)".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "..".into(),
                    command: "cd ..".into(),
                    description: Some("Go up one directory".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "cc".into(),
                    command: "cd ~".into(),
                    description: Some("Change to home directory".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: ".".into(),
                    command: "source .".into(),
                    description: Some("Source local environment file".into()),
                    execute: true,
                },
                // Git
                Shortcut {
                    sequence: "gs".into(),
                    command: "git status".into(),
                    description: Some("Git status".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "ga".into(),
                    command: "git add .".into(),
                    description: Some("Git add all".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "gc".into(),
                    command: "git commit -m \"".into(),
                    description: Some("Start a Git commit".into()),
                    execute: false,
                },
                Shortcut {
                    sequence: "gp".into(),
                    command: "git push".into(),
                    description: Some("Git push".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "gl".into(),
                    command: "git log --oneline".into(),
                    description: Some("Compact Git log".into()),
                    execute: true,
                },
                // System utilities
                Shortcut {
                    sequence: "rm".into(),
                    command: "rm -r ".into(),
                    description: Some("Remove file".into()),
                    execute: false,
                },
                Shortcut {
                    sequence: "h".into(),
                    command: "htop".into(),
                    description: Some("System monitor".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "p".into(),
                    command: "ping google.com".into(),
                    description: Some("Ping Google".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "df".into(),
                    command: "df -h".into(),
                    description: Some("Disk usage".into()),
                    execute: true,
                },
                // Networking
                Shortcut {
                    sequence: "ip".into(),
                    command: "ip a".into(),
                    description: Some("Show IP addresses".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "ss".into(),
                    command: "ss -tuln".into(),
                    description: Some("Show open sockets and ports".into()),
                    execute: true,
                },
            ],
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_shortcuts() -> Vec<Shortcut> {
        vec![
            Shortcut {
                sequence: "gs".into(),
                command: "git status".into(),
                description: None,
                execute: true,
            },
            Shortcut {
                sequence: "v".into(),
                command: "vim ".into(),
                description: None,
                execute: false,
            },
        ]
    }

    #[test]
    fn test_exact_match() {
        let shortcuts = test_shortcuts();
        let manager = ShortcutManager::new(shortcuts);

        let result = manager.match_sequence("gs");
        assert!(result.is_some());
        assert!(result.unwrap().execute);

        let result = manager.match_sequence("v");
        assert!(result.is_some());
        assert!(!result.unwrap().execute);

        let result = manager.match_sequence("x");
        assert!(result.is_none());

        let result = manager.match_sequence("g");
        assert!(result.is_none());
    }

    #[test]
    fn test_partial_match() {
        let shortcuts = test_shortcuts();
        let manager = ShortcutManager::new(shortcuts);

        assert!(manager.has_partial_match("g"));
        assert!(!manager.has_partial_match("x"));
    }
}
