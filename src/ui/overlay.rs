use crossterm::{cursor, terminal, QueueableCommand};
use std::io::Write;

use crate::{error::LeadrError, types::Shortcuts};

pub struct Overlay {
    height: u16,
    scroll_up: u16,
}

impl Overlay {
    pub fn try_new(overlay_height: u16) -> Result<Self, LeadrError> {
        let mut tty = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;

        let (_cols, rows) = terminal::size()?;
        let cursor_line = std::env::var("LEADR_CURSOR_LINE")?.parse::<u16>()?;

        let lines_below = rows.saturating_sub(cursor_line);
        let scroll_up = overlay_height.saturating_sub(lines_below);

        if scroll_up > 0 {
            tty.queue(terminal::ScrollUp(scroll_up))?
                .queue(cursor::MoveUp(scroll_up))?;
        }

        tty.flush()?;

        Ok(Self {
            height: overlay_height,
            scroll_up,
        })
    }

    pub fn clear(&self) -> std::io::Result<()> {
        let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        let (_cols, rows) = terminal::size()?;
        let start_y = rows.saturating_sub(self.height);

        stdout
            .queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(0, start_y))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(cursor::RestorePosition)?;

        if self.scroll_up > 0 {
            stdout
                .queue(terminal::ScrollDown(self.scroll_up))?
                .queue(cursor::MoveDown(self.scroll_up))?;
        }
        stdout.flush()
    }

    pub fn draw(&self, sequence: &str, shortcuts: &Shortcuts) -> Result<(), LeadrError> {
        let mut tty = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        let (cols, rows) = terminal::size()?;
        let start_y = rows.saturating_sub(self.height);

        tty.queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(0, start_y))?;

        write!(
            tty,
            "╭{:─<col$}╮",
            "",
            col = cols.saturating_sub(2) as usize
        )?;

        tty.queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(0, rows.saturating_sub(1)))?;

        write!(
            tty,
            "╰{:─<col$}╯",
            "",
            col = cols.saturating_sub(2) as usize
        )?;

        tty.queue(cursor::RestorePosition)?;

        tty.flush()?;
        Ok(())
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}
