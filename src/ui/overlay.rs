use std::io::Write;

use crossterm::{
    cursor,
    style::{Color, Stylize},
    terminal, QueueableCommand,
};
use serde::{Deserialize, Serialize};

use crate::{error::LeadrError, types::Shortcuts};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<RgbColor> for crossterm::style::Color {
    fn from(rgb: RgbColor) -> Self {
        crossterm::style::Color::Rgb {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum BorderType {
    #[default]
    Rounded,
    Square,
    Top,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub height: u16,
    pub bg_color: RgbColor,
    pub frame_color: RgbColor,
    pub border: BorderType,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            height: 10,
            bg_color: RgbColor { r: 49, g: 50, b: 68 },
            frame_color: RgbColor { r: 137, g: 180, b: 250 },
            border: BorderType::Rounded,
        }
    }
}

pub struct Overlay {
    pub config: Config,
    scroll_up: u16,
}

impl Overlay {
    pub fn try_new(config: Config) -> Result<Self, LeadrError> {
        let mut tty = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;

        let (_cols, rows) = terminal::size()?;
        let cursor_line = std::env::var("LEADR_CURSOR_LINE")?.parse::<u16>()?;

        let lines_below = rows.saturating_sub(cursor_line);
        let scroll_up = config.height.saturating_sub(lines_below);

        if scroll_up > 0 {
            tty.queue(terminal::ScrollUp(scroll_up))?
                .queue(cursor::MoveUp(scroll_up))?;
        }

        tty.flush()?;

        Ok(Self {
            config,
            scroll_up,
        })
    }

    pub fn clear(&self) -> std::io::Result<()> {
        let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        let (_cols, rows) = terminal::size()?;
        let start_y = rows.saturating_sub(self.config.height);

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
        let start_y = rows.saturating_sub(self.config.height);

        tty.queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(0, start_y))?;

        let top = format!("╭{:─<col$}╮", "", col = cols.saturating_sub(2) as usize)
            .with(Color::Rgb {
                r: 0,
                g: 128,
                b: 255,
            })
            .on(Color::Rgb {
                r: 10,
                g: 10,
                b: 50,
            });
        write!(tty, "{}", top)?;

        tty.queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(0, rows.saturating_sub(1)))?;

        let bottom = format!("╰{:─<col$}╯", "", col = cols.saturating_sub(2) as usize)
            .with(Color::Rgb {
                r: 0,
                g: 128,
                b: 255,
            })
            .on(Color::Rgb {
                r: 10,
                g: 10,
                b: 50,
            });
        write!(tty, "{}", bottom)?;

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
