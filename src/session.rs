use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::{input::RawModeGuard, Config, LeadrError, Mappings, Panel, Theme};

pub enum SessionResult {
    Command(String),
    Cancelled,
    NoMatch,
}

/// Handles keyboard input and matches sequences to mapped commands.
pub struct LeadrSession {
    mappings: Mappings,
    config: Config,
    theme: Theme,
    sequence: String,
}

impl LeadrSession {
    pub fn new(mappings: Mappings, config: Config, theme: Theme) -> Self {
        LeadrSession {
            mappings,
            config,
            theme,
            sequence: String::new(),
        }
    }

    /// Runs the input loop, capturing key events and returning when a mapping is matched,
    /// canceled, or an invalid sequence is entered.
    pub fn run(&mut self) -> Result<SessionResult, LeadrError> {
        let _guard = RawModeGuard::new()?;
        let start_time = Instant::now();
        let mut panel: Option<Panel> = None;

        loop {
            let timeout_reached = start_time.elapsed() >= self.config.panel.timeout;
            if self.config.panel.enabled && panel.is_none() && timeout_reached {
                panel =
                    Panel::try_new(self.config.panel.clone(), self.theme.clone()).ok();
                if let Some(panel) = panel.as_mut() {
                    let _ = panel.draw(&self.sequence, &self.mappings);
                }
            }

            if poll(Duration::from_millis(50))? {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = read()?
                {
                    if modifiers == crossterm::event::KeyModifiers::CONTROL {
                        if code == KeyCode::Char('c') {
                            return Ok(SessionResult::Cancelled);
                        }
                        continue;
                    }
                    match code {
                        KeyCode::Char(c) => {
                            self.sequence.push(c);
                            if let Some(mapping) = self.mappings.match_sequence(&self.sequence) {
                                return Ok(SessionResult::Command(mapping.format_command()));
                            }

                            if !self.mappings.has_partial_match(&self.sequence) {
                                return Ok(SessionResult::NoMatch);
                            }
                        }
                        KeyCode::Backspace => {
                            self.sequence.pop();
                        }
                        KeyCode::Esc => {
                            return Ok(SessionResult::Cancelled);
                        }
                        _ => {}
                    }
                    if let Some(panel) = panel.as_mut() {
                        let _ = panel.draw(&self.sequence, &self.mappings);
                    }
                }
            }
        }
    }
}
