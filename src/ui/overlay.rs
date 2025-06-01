use std::{collections::HashMap, io::Write};

use crossterm::{QueueableCommand, cursor, style::Stylize, terminal};
use serde::{Deserialize, Serialize};

use crate::{
    Shortcut, Shortcuts,
    error::LeadrError,
    ui::{
        area::{Area, ColumnLayout},
        color::RgbColor,
        entry::{Config as EntryConfig, Entry},
    },
};

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
    pub column_layout: ColumnLayout,
    pub entry_config: EntryConfig,
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
            column_layout: ColumnLayout::default(),
            entry_config: EntryConfig::default(),
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

        let outer_area = Area {
            x: self.config.padding,
            y: start_y,
            width: cols.saturating_sub(2 * self.config.padding),
            height: self.config.height,
        };

        self.draw_border(&mut tty, &outer_area)?;

        let next_options = group_next_options(sequence, shortcuts);
        let mut keys: Vec<_> = next_options.keys().collect();
        keys.sort();
        let inner_area = Area {
            x: outer_area.x + 1,
            y: outer_area.y + 1,
            width: outer_area.width.saturating_sub(2),
            height: outer_area.height.saturating_sub(2),
        };

        let required_num_columns = (keys.len() as f64 / inner_area.height as f64).ceil() as u16;
        let columns =
            inner_area.split_horizontally(&self.config.column_layout, &required_num_columns);
        for (i, column) in columns.iter().enumerate() {
            let column_keys = keys
                .iter()
                .skip(i * column.height as usize)
                .take(column.height as usize)
                .cloned()
                .collect::<Vec<_>>();
            self.draw_entries(&mut tty, column, &next_options, &column_keys)?;
        }

        Ok(())
    }

    fn draw_border(&self, tty: &mut std::fs::File, area: &Area) -> std::io::Result<()> {
        let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) =
            match self.config.border {
                BorderType::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
                BorderType::Square => ('┌', '┐', '└', '┘', '─', '│'),
                BorderType::Top => ('─', '─', ' ', ' ', '─', ' '),
                BorderType::None => (' ', ' ', ' ', ' ', ' ', ' '),
            };

        let inner_width = area.width.saturating_sub(2);
        let horizontal_line = horizontal.to_string().repeat(inner_width.into());

        tty.queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(area.x, area.y))?;

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
            tty.queue(cursor::MoveTo(area.x, area.y + i))?;
            let line = format!(
                "{vl}{space}{vr}",
                space = " ".repeat(inner_width.into()),
                vl = vertical,
                vr = vertical
            )
            .with(self.config.border_color.into())
            .on(self.config.bg_color.into());
            write!(tty, "{}", line)?;
        }

        // Bottom border
        if matches!(self.config.border, BorderType::Rounded | BorderType::Square) {
            tty.queue(cursor::MoveTo(area.x, area.y + area.height))?;
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
        area: &Area,
        next_options_map: &HashMap<String, Vec<&Shortcut>>,
        keys: &Vec<&String>,
    ) -> std::io::Result<()> {
        let mut line = area.y;

        for key in keys {
            if line >= area.y + area.height {
                break; // stop if no more vertical space
            }
            tty.queue(cursor::MoveTo(area.x, line))?;

            let shortcuts = &next_options_map[*key];
            let stylized_entry = Entry::new(key, shortcuts, area.width, &self.config.entry_config);
            stylized_entry.to_tty(tty)?;

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
