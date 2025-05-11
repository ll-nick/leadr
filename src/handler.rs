use crate::input::RawModeGuard;
use crate::models::{LeadrError, Shortcut, ShortcutResult};
use crossterm::event::{Event, KeyCode, KeyEvent, read};
use std::collections::HashMap;

pub struct ShortcutHandler {
    shortcuts: HashMap<String, Shortcut>,
    sequence: String,
}

impl ShortcutHandler {
    pub fn new(shortcuts: Vec<Shortcut>) -> Self {
        let mut map = HashMap::new();
        for shortcut in shortcuts {
            map.insert(shortcut.sequence.clone(), shortcut);
        }
        ShortcutHandler {
            shortcuts: map,
            sequence: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<ShortcutResult, LeadrError> {
        let _guard = RawModeGuard::new().map_err(LeadrError::TerminalSetup)?;

        loop {
            if let Event::Key(KeyEvent { code, modifiers, .. }) =
                read().map_err(|e| LeadrError::ReadError(e.to_string()))?
            {
                if modifiers == crossterm::event::KeyModifiers::CONTROL {
                    if code == KeyCode::Char('c') {
                        return Ok(ShortcutResult::Cancelled);
                    }
                    continue;
                }
                match code {
                    KeyCode::Char(c) => {
                        self.sequence.push(c);
                        if let Some(shortcut) = self.match_sequence(&self.sequence) {
                            return Ok(ShortcutResult::Shortcut(shortcut.clone()));
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
        let manager = ShortcutHandler::new(shortcuts);

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
        let manager = ShortcutHandler::new(shortcuts);

        assert!(manager.has_partial_match("g"));
        assert!(!manager.has_partial_match("x"));
    }
}
