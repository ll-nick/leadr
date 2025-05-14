use std::{collections::HashMap, io::Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    terminal, QueueableCommand,
};

use crate::{
    input::RawModeGuard,
    types::{Shortcut, ShortcutResult},
    LeadrError,
};

pub struct ShortcutHandler {
    shortcuts: HashMap<String, Shortcut>,
    padding: usize,
    sequence: String,
}

impl ShortcutHandler {
    pub fn new(shortcuts: HashMap<String, Shortcut>, padding: usize) -> Self {
        ShortcutHandler {
            shortcuts,
            padding,
            sequence: String::new(),
        }
    }

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
                        let _ = self.print_sequence_bottom_right();
                        if let Some(shortcut) = self.match_sequence(&self.sequence) {
                            return Ok(ShortcutResult::Shortcut(shortcut.clone()));
                        }

                        if !self.has_partial_match(&self.sequence) {
                            return Ok(ShortcutResult::NoMatch);
                        }
                    }
                    KeyCode::Backspace => {
                        self.sequence.pop();
                        let _ = self.print_sequence_bottom_right();
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

    fn print_sequence_bottom_right(&self) -> std::io::Result<()> {
        let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        let sequence = &self.sequence;

        let (cols, rows) = terminal::size().unwrap_or((80, 24));
        let max_len = sequence.len().min(cols as usize);
        let x = cols.saturating_sub((max_len + self.padding) as u16);
        let y = rows.saturating_sub(1);

        stdout
            .queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(x, y))?
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
            .queue(cursor::Hide)?;

        write!(stdout, "{}", &sequence[..max_len])?;

        stdout
            .queue(cursor::Show)?
            .queue(cursor::RestorePosition)?
            .flush()?;

        Ok(())
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
            });
        shortcuts.insert(
            "v".into(),
            Shortcut {
                command: "vim ".into(),
                description: None,
                execute: false,
            });
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
