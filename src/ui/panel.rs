use std::{io::Write, time::Duration};

use color_eyre::eyre::Result;
use crossterm::{QueueableCommand, cursor, style::Stylize, terminal};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    Mappings, Symbols, Theme,
    cursor::query_cursor_position,
    ui::{
        area::{Area, ColumnLayout},
        entry::Entry,
    },
};

fn duration_as_milliseconds<S>(dur: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(dur.as_millis() as u64)
}

fn duration_from_milliseconds<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let ms = u64::deserialize(d)?;
    Ok(Duration::from_millis(ms))
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
    pub enabled: bool,
    #[serde(
        rename = "delay_ms",
        serialize_with = "duration_as_milliseconds",
        deserialize_with = "duration_from_milliseconds"
    )]
    pub delay: Duration,
    pub fail_silently: bool,
    pub theme_name: String,
    pub layout: LayoutConfig,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            delay: Duration::from_millis(500),
            fail_silently: true,
            theme_name: "catppuccin-mocha".into(),
            layout: LayoutConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub border_type: BorderType,
    pub columns: ColumnLayout,
    pub height: u16,
    pub padding: u16,
    pub symbols: Symbols,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            border_type: BorderType::Rounded,
            columns: ColumnLayout::default(),
            height: 10,
            padding: 2,
            symbols: Symbols::default(),
        }
    }
}

pub struct Panel {
    pub config: Config,
    pub theme: Theme,
    scroll_up: u16,
}

impl Panel {
    pub fn try_new(config: Config, theme: Theme) -> Result<Self> {
        let mut tty = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;

        let (_cols, rows) = terminal::size()?;
        let (_cursor_x, cursor_y) = query_cursor_position()?;
        let line_below_cursor = cursor_y.saturating_add(1);

        let lines_below = rows.saturating_sub(line_below_cursor);
        let scroll_up = config.layout.height.saturating_sub(lines_below);

        if scroll_up > 0 {
            tty.queue(terminal::ScrollUp(scroll_up))?
                .queue(cursor::MoveUp(scroll_up))?;
        }

        tty.flush()?;

        Ok(Self {
            config,
            theme,
            scroll_up,
        })
    }

    pub fn clear(&self) -> std::io::Result<()> {
        let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        let (_cols, rows) = terminal::size()?;
        let start_y = rows.saturating_sub(self.config.layout.height);

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

    pub fn draw(&self, sequence: &str, mappings: &Mappings) -> Result<()> {
        let mut tty = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        let (cols, rows) = terminal::size()?;
        let start_y = rows.saturating_sub(self.config.layout.height);

        let outer_area = Area {
            x: self.config.layout.padding,
            y: start_y,
            width: cols.saturating_sub(2 * self.config.layout.padding),
            height: self.config.layout.height,
        };

        tty.queue(cursor::SavePosition)?;
        self.draw_border(&mut tty, &outer_area)?;
        let border_width = 1;
        let footer_height = 2;

        let next_possible_keys = mappings.next_possible_keys(sequence);
        let entry_area = Area {
            x: outer_area.x + 1,
            y: outer_area.y + 1,
            width: outer_area.width.saturating_sub(2 * border_width),
            height: outer_area
                .height
                .saturating_sub(2 * border_width + footer_height),
        };

        let required_num_columns =
            (next_possible_keys.len() as f64 / entry_area.height as f64).ceil() as u16;
        let columns =
            entry_area.split_horizontally(&self.config.layout.columns, &required_num_columns);
        for (i, column) in columns.iter().enumerate() {
            let column_keys = next_possible_keys
                .iter()
                .skip(i * column.height as usize)
                .take(column.height as usize)
                .cloned()
                .collect::<Vec<_>>();
            self.draw_entries(&mut tty, column, &mappings, &sequence, &column_keys)?;
        }

        let footer_area = Area {
            x: outer_area.x + 1,
            y: outer_area.y + outer_area.height - footer_height,
            width: outer_area.width.saturating_sub(2 * border_width),
            height: footer_height,
        };
        self.draw_footer(&mut tty, &footer_area, sequence)?;
        tty.queue(cursor::RestorePosition)?;

        Ok(())
    }

    fn draw_border(&self, tty: &mut std::fs::File, area: &Area) -> std::io::Result<()> {
        let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) =
            match self.config.layout.border_type {
                BorderType::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
                BorderType::Square => ('┌', '┐', '└', '┘', '─', '│'),
                BorderType::Top => ('─', '─', ' ', ' ', '─', ' '),
                BorderType::None => (' ', ' ', ' ', ' ', ' ', ' '),
            };

        let inner_width = area.width.saturating_sub(2);
        let horizontal_line = horizontal.to_string().repeat(inner_width.into());

        tty.queue(cursor::MoveTo(area.x, area.y))?;

        // Top border
        if !matches!(self.config.layout.border_type, BorderType::None) {
            let top = format!(
                "{tl}{line}{tr}",
                line = horizontal_line,
                tl = top_left,
                tr = top_right,
            )
            .with(self.theme.accent.into())
            .on(self.theme.background.into());
            write!(tty, "{}", top)?;
        }

        // Vertical sides
        for i in 1..self.config.layout.height {
            tty.queue(cursor::MoveTo(area.x, area.y + i))?;
            let line = format!(
                "{vl}{space}{vr}",
                space = " ".repeat(inner_width.into()),
                vl = vertical,
                vr = vertical
            )
            .with(self.theme.accent.into())
            .on(self.theme.background.into());
            write!(tty, "{}", line)?;
        }

        // Bottom border
        if matches!(
            self.config.layout.border_type,
            BorderType::Rounded | BorderType::Square
        ) {
            tty.queue(cursor::MoveTo(area.x, area.y + area.height))?;
            let bottom = format!(
                "{bl}{line}{br}",
                line = horizontal_line,
                bl = bottom_left,
                br = bottom_right
            )
            .with(self.theme.accent.into())
            .on(self.theme.background.into());
            write!(tty, "{}", bottom)?;
        }

        tty.flush()?;

        Ok(())
    }

    pub fn draw_entries(
        &self,
        tty: &mut std::fs::File,
        area: &Area,
        mappings: &Mappings,
        sequence: &str,
        keys: &Vec<String>,
    ) -> std::io::Result<()> {
        let mut line = area.y;

        for key in keys {
            if line >= area.y + area.height {
                break; // stop if no more vertical space
            }
            tty.queue(cursor::MoveTo(area.x, line))?;

            let full_sequence = format!("{sequence}{key}");
            let match_type = mappings.match_partial_sequence(&full_sequence);

            let stylized_entry = Entry::new(
                key,
                match_type,
                area.width,
                &self.config.layout.symbols,
                &self.theme,
            );
            stylized_entry.to_tty(tty)?;

            line += 1;
        }

        Ok(())
    }

    pub fn draw_footer(
        &self,
        tty: &mut std::fs::File,
        area: &Area,
        sequence: &str,
    ) -> std::io::Result<()> {
        let help_text = "󱊷  close  󰁮  back";
        let styled_help_text = help_text
            .with(self.theme.text_primary.into())
            .on(self.theme.background.into());
        let center_x = area.x + (area.width.saturating_sub(help_text.chars().count() as u16)) / 2;
        tty.queue(cursor::MoveTo(center_x, area.y))?;
        write!(tty, "{}", styled_help_text)?;

        tty.queue(cursor::MoveTo(area.x, area.y))?;
        let arrow = self
            .config
            .layout
            .symbols
            .sequence_begin
            .to_string()
            .with(self.theme.text_secondary.into())
            .on(self.theme.background.into());
        write!(tty, "{}", arrow)?;
        let sequence_text = sequence
            .to_string()
            .with(self.theme.text_primary.into())
            .on(self.theme.background.into());
        write!(tty, "{}", sequence_text)?;

        Ok(())
    }
}

impl Drop for Panel {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}
