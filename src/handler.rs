use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, KeyEvent, poll, read};

use crate::{
    Config, LeadrError, Theme,
    input::RawModeGuard,
    types::{Mapping, LeadrResult},
    ui::overlay::Overlay,
};

/// Handles keyboard input and matches sequences to mapped commands.
pub struct LeadrSession {
    config: Config,
    theme: Theme,
    sequence: String,
}

impl LeadrSession {
    pub fn new(config: Config, theme: Theme) -> Self {
        LeadrSession {
            config,
            theme,
            sequence: String::new(),
        }
    }

    /// Runs the input loop, capturing key events and returning when a mapping is matched,
    /// cancelled, or an invalid sequence is entered.
    pub fn run(&mut self) -> Result<LeadrResult, LeadrError> {
        let _guard = RawModeGuard::new()?;
        let start_time = Instant::now();
        let mut overlay: Option<Overlay> = None;

        loop {
            let timeout_reached = start_time.elapsed() >= self.config.overlay_timeout;
            if self.config.show_overlay && overlay.is_none() && timeout_reached {
                overlay =
                    Overlay::try_new(self.config.overlay_style.clone(), self.theme.clone()).ok();
                if let Some(overlay) = overlay.as_mut() {
                    let _ = overlay.draw(&self.sequence, &self.config.mappings);
                }
            }

            if poll(Duration::from_millis(50))? {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = read()?
                {
                    if modifiers == crossterm::event::KeyModifiers::CONTROL {
                        if code == KeyCode::Char('c') {
                            return Ok(LeadrResult::Cancelled);
                        }
                        continue;
                    }
                    match code {
                        KeyCode::Char(c) => {
                            self.sequence.push(c);
                            if let Some(mapping) = self.match_sequence(&self.sequence) {
                                return Ok(LeadrResult::Command(mapping.format_command()));
                            }

                            if !self.has_partial_match(&self.sequence) {
                                return Ok(LeadrResult::NoMatch);
                            }
                        }
                        KeyCode::Backspace => {
                            self.sequence.pop();
                        }
                        KeyCode::Esc => {
                            return Ok(LeadrResult::Cancelled);
                        }
                        _ => {}
                    }
                    if let Some(overlay) = overlay.as_mut() {
                        let _ = overlay.draw(&self.sequence, &self.config.mappings);
                    }
                }
            }
        }
    }

    /// Returns an exact match for a given sequence, if one exists.
    fn match_sequence(&self, seq: &str) -> Option<&Mapping> {
        self.config.mappings.get(seq)
    }

    /// Returns true if any mapping begins with the given sequence.
    fn has_partial_match(&self, seq: &str) -> bool {
        self.config.mappings.keys().any(|k| k.starts_with(seq))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{InsertType, Mappings};

    fn test_config() -> Config {
        let mut mappings = Mappings::new();
        mappings.insert(
            "gs".into(),
            Mapping {
                command: "git status".into(),
                description: None,
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: false,
            },
        );
        mappings.insert(
            "s".into(),
            Mapping {
                command: "sudo ".into(),
                description: None,
                insert_type: InsertType::Prepend,
                evaluate: false,
                execute: false,
            },
        );

        Config {
            mappings,
            ..Default::default()
        }
    }

    #[test]
    fn test_exact_match() {
        let manager = LeadrSession::new(test_config(), Theme::default());

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
        let manager = LeadrSession::new(test_config(), Theme::default());

        assert!(manager.has_partial_match("g"));
        assert!(!manager.has_partial_match("x"));
    }
}
