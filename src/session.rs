use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, poll, read};

use crate::{Config, Mappings, Panel, RawModeGuard, Theme, ui::prompt};

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
    pub fn run(&mut self) -> Result<SessionResult> {
        let _raw_mode_guard = RawModeGuard::new()?;
        let start_time = Instant::now();
        let mut panel: Option<Panel> = None;

        // Cosmetically fix the prompt line disappearing while leadr is active.
        let mut prompt_guard = prompt::PromptGuard::try_new();
        if let Ok(ref mut guard) = prompt_guard
            && self.config.redraw_prompt_line
        {
            guard.redraw()?;
        }

        loop {
            let timeout_reached = start_time.elapsed() >= self.config.panel.delay;
            if self.config.panel.enabled && panel.is_none() && timeout_reached {
                panel = self.try_new_panel()?;
            }

            if poll(Duration::from_millis(50))?
                && let Event::Key(KeyEvent {
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

                if let Some(ref mut p) = panel {
                    self.try_draw_panel(p)?;
                }
            }
        }
    }

    /// Try creating a new panel and draw upon success.
    /// Will return Ok(None) if panel creation fails but fail_silently is set.
    fn try_new_panel(&self) -> Result<Option<Panel>> {
        match Panel::try_new(self.config.panel.clone(), self.theme.clone()) {
            Ok(mut p) => {
                self.try_draw_panel(&mut p)?;
                Ok(Some(p))
            }
            Err(_) if self.config.panel.fail_silently => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Try drawing the panel, respecting the fail_silently setting.
    fn try_draw_panel(&self, panel: &mut Panel) -> Result<()> {
        match panel.draw(&self.sequence, &self.mappings) {
            Ok(()) => Ok(()),
            Err(_) if self.config.panel.fail_silently => Ok(()),
            Err(e) => Err(e),
        }
    }
}
