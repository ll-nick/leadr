use std::io::Write;
use std::time::{Duration, Instant};

use crossterm::QueueableCommand;
use crossterm::event::{Event, KeyCode, KeyEvent, poll, read};
use unicode_width::UnicodeWidthStr;

use crate::{Config, LeadrError, Mappings, Panel, Theme, input::RawModeGuard};

fn visible_width(s: &str) -> usize {
    // Remove ANSI escapes
    let stripped_ansi = strip_ansi_escapes::strip(s);

    // Convert back to string
    let clean = String::from_utf8_lossy(&stripped_ansi);

    // Compute terminal width
    UnicodeWidthStr::width(clean.as_ref())
}

fn redraw_line() -> Result<(), LeadrError> {
    let mut tty = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
    let cursor_line = std::env::var("LEADR_CURSOR_LINE")?.parse::<u16>()?;
    let cursor_column = std::env::var("LEADR_CURSOR_COLUMN")?.parse::<u16>()?;
    let prompt = std::env::var("LEADR_PROMPT")?;
    let input = std::env::var("LEADR_CURRENT_INPUT")?;
    let prompt_width = visible_width(&prompt) as u16;

    tty.queue(crossterm::cursor::MoveTo(0, cursor_line))?
        .queue(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::CurrentLine,
        ))?
        .queue(crossterm::style::Print(format!("{}{}", prompt, input)))?
        .queue(crossterm::cursor::MoveTo(
            prompt_width + cursor_column,
            cursor_line,
        ))?
        .flush()?;

    Ok(())
}

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

        let _ = redraw_line();

        loop {
            let timeout_reached = start_time.elapsed() >= self.config.panel.timeout;
            if self.config.panel.enabled && panel.is_none() && timeout_reached {
                let result = (|| {
                    let p = Panel::try_new(self.config.panel.clone(), self.theme.clone())?;
                    p.draw(&self.sequence, &self.mappings)?;
                    Ok(p)
                })();

                match result {
                    Ok(p) => panel = Some(p),
                    Err(_e) if self.config.panel.fail_silently => {}
                    Err(e) => return Err(e),
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
                        if let Err(e) = panel.draw(&self.sequence, &self.mappings) {
                            if !self.config.panel.fail_silently {
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }
    }
}
