use std::collections::HashMap;

use crossterm::event::{read, Event, KeyCode, KeyEvent};

use crate::{
    input::RawModeGuard,
    types::{Shortcut, ShortcutResult},
    ui::Ui,
    LeadrError,
};

/// Handles keyboard input and matches sequences to configured shortcuts.
pub struct ShortcutHandler {
    shortcuts: HashMap<String, Shortcut>,
    padding: usize,
    sequence: String,
}

impl ShortcutHandler {
    /// Creates a new `ShortcutHandler` with given shortcuts and padding.
    ///
    /// `padding` controls how far from the right edge the input sequence is displayed.
    pub fn new(shortcuts: HashMap<String, Shortcut>, padding: usize) -> Self {
        ShortcutHandler {
            shortcuts,
            padding,
            sequence: String::new(),
        }
    }

    /// Runs the input loop, capturing key events and returning when a shortcut is matched,
    /// cancelled, or an invalid sequence is entered.
    pub fn run(&mut self) -> Result<ShortcutResult, LeadrError> {
        let _guard = RawModeGuard::new()?;
        let ui = Ui::new(true, self.padding);

        loop {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = read().map_err(LeadrError::InputReadError)?
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
                        let _ = ui.update(&self.sequence);
                        if let Some(shortcut) = self.match_sequence(&self.sequence) {
                            return Ok(ShortcutResult::Shortcut(shortcut.clone()));
                        }

                        if !self.has_partial_match(&self.sequence) {
                            return Ok(ShortcutResult::NoMatch);
                        }
                    }
                    KeyCode::Backspace => {
                        self.sequence.pop();
                        let _ = ui.update(&self.sequence);
                    }
                    KeyCode::Esc => {
                        return Ok(ShortcutResult::Cancelled);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Returns an exact match for a given sequence, if one exists.
    fn match_sequence(&self, seq: &str) -> Option<&Shortcut> {
        self.shortcuts.get(seq)
    }

    /// Returns true if any shortcut begins with the given sequence.
    fn has_partial_match(&self, seq: &str) -> bool {
        self.shortcuts.keys().any(|k| k.starts_with(seq))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_shortcuts() -> HashMap<String, Shortcut> {
        let mut shortcuts = HashMap::new();
        shortcuts.insert(
            "gs".into(),
            Shortcut {
                command: "git status".into(),
                description: None,
                execute: true,
            },
        );
        shortcuts.insert(
            "v".into(),
            Shortcut {
                command: "vim ".into(),
                description: None,
                execute: false,
            },
        );
        shortcuts
    }

    #[test]
    fn test_exact_match() {
        let shortcuts = test_shortcuts();
        let manager = ShortcutHandler::new(shortcuts, 0);

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
        let manager = ShortcutHandler::new(shortcuts, 0);

        assert!(manager.has_partial_match("g"));
        assert!(!manager.has_partial_match("x"));
    }
}
