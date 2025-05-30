use std::{collections::HashMap, io::Write};

use crossterm::{cursor, style::Stylize, terminal, QueueableCommand};
use serde::{Deserialize, Serialize};

use crate::{error::LeadrError, Shortcut, Shortcuts};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BorderType {
    Rounded,
    Square,
    Top,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub height: u16,
    pub bg_color: RgbColor,
    pub border_color: RgbColor,
    pub border: BorderType,
    pub padding: u16,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            height: 10,
            bg_color: RgbColor {
                r: 16,
                g: 16,
                b: 26,
            },
            border_color: RgbColor {
                r: 137,
                g: 180,
                b: 250,
            },
            border: BorderType::Rounded,
            padding: 2,
        }
    }
}

pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
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

        Ok(Self { config, scroll_up })
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

        self.draw_border(&mut tty, cols, start_y)?;

        let next_options = group_next_options(sequence, shortcuts);
        let inner_area = Area {
            x: self.config.padding + 2,
            y: start_y + 1,
            width: cols.saturating_sub(2 * self.config.padding + 4),
            height: self.config.height.saturating_sub(2),
        };

        self.draw_entries(&mut tty, inner_area, &next_options)?;


        Ok(())
    }

    fn draw_border(&self, tty: &mut std::fs::File, cols: u16, start_y: u16) -> std::io::Result<()> {
        let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) =
            match self.config.border {
                BorderType::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
                BorderType::Square => ('┌', '┐', '└', '┘', '─', '│'),
                BorderType::Top => ('─', '─', ' ', ' ', '─', ' '),
                BorderType::None => (' ', ' ', ' ', ' ', ' ', ' '),
            };

        let padding = self.config.padding as usize;
        let total_width = cols.saturating_sub(2) as usize;
        let content_width = total_width.saturating_sub(padding * 2);
        let horizontal_line = horizontal.to_string().repeat(content_width);

        tty.queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(padding as u16, start_y))?;

        // Top border
        if !matches!(self.config.border, BorderType::None) {
            let top = format!(
                "{tl}{line}{tr}",
                line = horizontal_line,
                tl = top_left,
                tr = top_right,
            )
            .with(self.config.border_color.into())
            .on(self.config.bg_color.into());
            write!(tty, "{}", top)?;
        }

        // Vertical sides
        for i in 1..self.config.height {
            tty.queue(cursor::MoveTo(padding as u16, start_y + i))?;
            let line = format!(
                "{vl}{space}{vr}",
                space = " ".repeat(content_width),
                vl = vertical,
                vr = vertical
            )
            .with(self.config.border_color.into())
            .on(self.config.bg_color.into());
            write!(tty, "{}", line)?;
        }

        // Bottom border
        if matches!(self.config.border, BorderType::Rounded | BorderType::Square) {
            tty.queue(cursor::MoveTo(padding as u16, start_y + self.config.height))?;
            let bottom = format!(
                "{bl}{line}{br}",
                line = horizontal_line,
                bl = bottom_left,
                br = bottom_right
            )
            .with(self.config.border_color.into())
            .on(self.config.bg_color.into());
            write!(tty, "{}", bottom)?;
        }

        tty.queue(cursor::RestorePosition)?;
        tty.flush()?;

        Ok(())
    }

    pub fn draw_entries(
        &self,
        tty: &mut std::fs::File,
        area: Area,
        next_options_map: &HashMap<String, Vec<&Shortcut>>,
    ) -> std::io::Result<()> {
        let mut line = area.y + 1; // start below border top

        let mut keys: Vec<_> = next_options_map.keys().collect();
        keys.sort();

        for key in keys {
            if line >= area.y + area.height {
                break; // stop if no more vertical space
            }

            let shortcuts = &next_options_map[key];
            let shortcut = shortcuts.first().unwrap();

            if shortcuts.len() > 1 {
                // If there are multiple shortcuts, we display the first one
                // and indicate that there are more options.
            }
            let label = if shortcuts.len() > 1 {
                format!(" ({} options)", shortcuts.len())
            } else {
                shortcut
                    .description
                    .as_deref()
                    .unwrap_or(&shortcut.command)
                    .to_string()
            };

            let text = format!("[{}] -> {}", key, label);

            // TODO: Truncate text if it exceeds the available width

            tty.queue(cursor::MoveTo(area.x, line))?;
            write!(
                tty,
                "{}",
                text.with(self.config.border_color.into())
                    .on(self.config.bg_color.into())
            )?;

            line += 1;
        }

        Ok(())
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}

pub fn group_next_options<'a>(
    sequence: &str,
    shortcuts: &'a Shortcuts,
) -> HashMap<String, Vec<&'a Shortcut>> {
    let mut map: HashMap<String, Vec<&Shortcut>> = HashMap::new();

    for (key, shortcut) in shortcuts.iter() {
        if key.starts_with(&sequence) {
            if let Some((_, char)) = key[sequence.len()..].char_indices().next() {
                // next_key is this character (handle utf-8)
                let next_key = char.to_string();
                map.entry(next_key).or_default().push(shortcut);
            }
        }
    }

    map
}
