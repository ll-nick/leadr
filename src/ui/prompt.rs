//! Line redraw module for Leadr.
//!
//! ## Why do we need to redraw the line at all?
//!
//! When you invoke a `bind -x` command in Bash (or similar in Zsh), the shell
//! suspends its line editor (GNU Readline, ZLE, etc.) and executes your hook
//! function/program. While your Rust program is running, the terminal is no
//! longer "owned" by the shell's line editor.
//! This means that the current line will be cleared and prompt and any previously
//! typed input will not be visible during the execution of leadr.
//!
//! To give the illusion that the shell never "let go" of the line, leadr must
//! explicitly repaint the prompt and input while it is active.
//!
//! This is wrapped into a RAII guard to ensure the cursor position is restored
//! cleanly when leadr exits, not matter how it exits.

use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use crossterm::{ExecutableCommand, QueueableCommand, cursor};
use unicode_width::UnicodeWidthStr;

use crate::LeadrError;

/// Compute the visible width of a string by stripping ANSI escape sequences
/// and measuring Unicode display width.
fn visible_width(s: &str) -> usize {
    // Strip ANSI escapes
    let stripped_ansi = strip_ansi_escapes::strip(s);

    // Convert back to UTF-8 safely
    let clean = String::from_utf8_lossy(&stripped_ansi);

    // Measure grapheme width in terminal cells
    UnicodeWidthStr::width(clean.as_ref())
}

/// Responsible for redrawing the shell line during the session.
/// RAII guard ensures cursor is reset on drop.
pub struct PromptGuard {
    tty: std::fs::File,
    cursor_line: u16,
    cursor_column: u16,
    prompt: String,
    input: String,
}

impl PromptGuard {
    /// Create a new redraw guard.
    pub fn try_new() -> Result<Self, LeadrError> {
        let tty = OpenOptions::new().write(true).open("/dev/tty")?;
        let cursor_line = env::var("LEADR_CURSOR_LINE")?.parse::<u16>()?;
        let cursor_column = env::var("LEADR_CURSOR_COLUMN")?.parse::<u16>()?;
        let prompt = env::var("LEADR_PROMPT")?;
        let input = env::var("LEADR_CURRENT_INPUT")?;

        Ok(Self {
            tty,
            cursor_line,
            cursor_column,
            prompt,
            input,
        })
    }

    /// Redraw the current prompt + input line, restoring cursor position.
    pub fn redraw(&mut self) -> Result<(), LeadrError> {
        let prompt_width = visible_width(&self.prompt) as u16;

        self.tty
            .queue(crossterm::cursor::MoveTo(0, self.cursor_line))?
            .queue(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine,
            ))?
            .queue(crossterm::style::Print(format!(
                "{}{}",
                self.prompt, self.input
            )))?
            .queue(crossterm::cursor::MoveTo(
                prompt_width + self.cursor_column,
                self.cursor_line,
            ))?
            .flush()?;

        Ok(())
    }
}

impl Drop for PromptGuard {
    fn drop(&mut self) {
        // On exit, force cursor back to col 0 on its line
        let _ = self.tty.execute(cursor::MoveTo(0, self.cursor_line));
    }
}
