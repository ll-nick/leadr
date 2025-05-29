use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::{
    input::RawModeGuard,
    types::{Shortcut, ShortcutResult, Shortcuts},
    ui::overlay::Overlay,
    Config, LeadrError,
};

/// Handles keyboard input and matches sequences to configured shortcuts.
pub struct ShortcutHandler {
    shortcuts: Shortcuts,
    sequence: String,
}

impl ShortcutHandler {
    /// Creates a new `ShortcutHandler` with given shortcuts and padding.
    ///
    /// `padding` controls how far from the right edge the input sequence is displayed.
    pub fn new(config: Config) -> Self {
        ShortcutHandler {
            shortcuts: config.shortcuts,
            sequence: String::new(),
        }
    }

    /// Runs the input loop, capturing key events and returning when a shortcut is matched,
    /// cancelled, or an invalid sequence is entered.
    pub fn run(&mut self) -> Result<ShortcutResult, LeadrError> {
        let _guard = RawModeGuard::new()?;
        let start_time = Instant::now();
        let mut overlay: Option<Overlay> = None;

        // TODO: Add to config
        let show_overlay = true;
        let overlay_timeout = Duration::from_millis(500);
        let overlay_height = 10;

        loop {
            let timeout_reached = start_time.elapsed() >= overlay_timeout;
            if show_overlay && overlay.is_none() && timeout_reached {
                overlay = Overlay::try_new(overlay_height).ok();
                if let Some(overlay) = overlay.as_mut() {
                    let _ = overlay.draw(&self.sequence, &self.shortcuts);
                }
            }

            if poll(Duration::from_millis(50)).map_err(LeadrError::InputReadError)? {
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
                    if let Some(overlay) = overlay.as_mut() {
                        let _ = overlay.draw(&self.sequence, &self.shortcuts);
                    }
                    match code {
                        KeyCode::Char(c) => {
                            self.sequence.push(c);
                            if let Some(shortcut) = self.match_sequence(&self.sequence) {
                                return Ok(ShortcutResult::Shortcut(shortcut.format_command()));
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
    use crate::types::{InsertType, Shortcuts};

    fn test_config() -> Config {
        let mut shortcuts = Shortcuts::new();
        shortcuts.insert(
            "gs".into(),
            Shortcut {
                command: "git status".into(),
                description: None,
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: false,
            },
        );
        shortcuts.insert(
            "s".into(),
            Shortcut {
                command: "sudo ".into(),
                description: None,
                insert_type: InsertType::Prepend,
                evaluate: false,
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
        assert_eq!(result.unwrap().insert_type, InsertType::Replace);

        let result = manager.match_sequence("s");
        assert!(result.is_some());
        assert_eq!(result.unwrap().insert_type, InsertType::Prepend);

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
