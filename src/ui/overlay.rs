use crossterm::{cursor, terminal, QueueableCommand};
use std::io::Write;

pub struct Overlay {
    height: u16,
    scroll_up: u16,
}

impl Overlay {
    pub fn new(overlay_height: u16) -> std::io::Result<Self> {
        let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;

        let (_cols, rows) = terminal::size()?;
        let cursor_line = std::env::var("LEADR_CURSOR_LINE")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(rows.saturating_sub(1));

        let lines_below = rows.saturating_sub(cursor_line + 1);
        let scroll_up = overlay_height.saturating_sub(lines_below);

        if scroll_up > 0 {
            stdout.queue(terminal::ScrollUp(scroll_up))?;
            stdout.queue(cursor::MoveUp(scroll_up))?;
        }

        stdout.flush()?;
        Ok(Self {
            height: overlay_height,
            scroll_up,
        })
    }

    pub fn clear(&self) -> std::io::Result<()> {
        let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        let (_cols, rows) = terminal::size().unwrap_or((80, 24));
        let start_y = rows.saturating_sub(self.height);

        stdout
            .queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(0, start_y))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(cursor::RestorePosition)?;

        if self.scroll_up > 0 {
            stdout.queue(terminal::ScrollDown(self.scroll_up))?
            .queue(cursor::MoveDown(self.scroll_up))?;
        }
        stdout.flush()
    }

    // Draw methods here...
}

impl Drop for Overlay {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}
