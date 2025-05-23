use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent, read};

use crate::{
    Config, EncodingStrings, LeadrError, Ui,
    input::RawModeGuard,
    types::{Shortcut, ShortcutResult},
};

/// Handles keyboard input and matches sequences to configured shortcuts.
pub struct ShortcutHandler {
    encoding_strings: EncodingStrings,
    shortcuts: HashMap<String, Shortcut>,
    sequence: String,
    ui: Ui,
}

impl ShortcutHandler {
    /// Creates a new `ShortcutHandler` with given shortcuts and padding.
    ///
    /// `padding` controls how far from the right edge the input sequence is displayed.
    pub fn new(config: Config) -> Self {
        ShortcutHandler {
            encoding_strings: config.encoding_strings,
            shortcuts: config.shortcuts,
            sequence: String::new(),
            ui: Ui::new(config.print_sequence, config.padding),
        }
    }

    /// Runs the input loop, capturing key events and returning when a shortcut is matched,
    /// cancelled, or an invalid sequence is entered.
    pub fn run(&mut self) -> Result<ShortcutResult, LeadrError> {
        let _guard = RawModeGuard::new()?;

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
                        let _ = self.ui.update(&self.sequence);
                        if let Some(shortcut) = self.match_sequence(&self.sequence) {
                            return Ok(ShortcutResult::Shortcut(
                                shortcut.format_command(&self.encoding_strings),
                            ));
                        }

                        if !self.has_partial_match(&self.sequence) {
                            return Ok(ShortcutResult::NoMatch);
                        }
                    }
                    KeyCode::Backspace => {
                        self.sequence.pop();
                        let _ = self.ui.update(&self.sequence);
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

    fn test_config() -> Config {
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

        Config {
            shortcuts,
            ..Default::default()
        }
    }

    #[test]
    fn test_exact_match() {
        let manager = ShortcutHandler::new(test_config());

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
        let manager = ShortcutHandler::new(test_config());

        assert!(manager.has_partial_match("g"));
        assert!(!manager.has_partial_match("x"));
    }
}
