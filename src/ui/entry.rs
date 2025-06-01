use std::io::Write;

use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};

use crate::{
    types::{InsertType, Shortcut},
    ui::color::RgbColor,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlagSymbols {
    pub replace: String,
    pub insert: String,
    pub append: String,
    pub prepend: String,
    pub surround: String,
    pub evaluate: String,
    pub execute: String,
}

impl std::default::Default for FlagSymbols {
    fn default() -> Self {
        Self {
            replace: " ".into(),
            insert: "".into(),
            append: "󰌒".into(),
            prepend: "󰌥".into(),
            surround: "󰅪".into(),
            evaluate: "󰊕".into(),
            execute: "󰌑".into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Colors {
    pub bg_color: RgbColor,
    pub key_color: RgbColor,
    pub arrow_color: RgbColor,
    pub label_color: RgbColor,
    pub more_options_color: RgbColor,
    pub flags_color: RgbColor,
}

impl std::default::Default for Colors {
    fn default() -> Self {
        Self {
            bg_color: RgbColor {
                r: 16,
                g: 16,
                b: 26,
            },
            key_color: RgbColor {
                r: 137,
                g: 180,
                b: 250,
            },
            arrow_color: RgbColor {
                r: 108,
                g: 112,
                b: 134,
            },
            label_color: RgbColor {
                r: 245,
                g: 224,
                b: 220,
            },
            more_options_color: RgbColor {
                r: 137,
                g: 180,
                b: 250,
            },
            flags_color: RgbColor {
                r: 250,
                g: 179,
                b: 135,
            },
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub flag_symbols: FlagSymbols,
    pub colors: Colors,
}

pub struct Entry {
    pub styled_parts: Vec<crossterm::style::StyledContent<String>>,
}

impl Entry {
    pub fn new(key: &str, shortcuts: &Vec<&Shortcut>, width: u16, config: &Config) -> Self {
        let more_options = shortcuts.len() != 1;
        let mut label = if more_options {
            format!("+{} shortcuts", shortcuts.len())
        } else {
            let shortcut = &shortcuts.first().unwrap();
            shortcut
                .description
                .as_deref()
                .unwrap_or(&shortcut.command)
                .to_string()
        };

        let flags = if more_options {
            " ".repeat(5) // make sure to take up space that flags would take
        } else {
            let shortcut = &shortcuts.first().unwrap();
            format_flags(shortcut, &config.flag_symbols)
        };

        let raw_entry = format!("{} → {} {}", key, label, flags);
        let raw_entry_width = raw_entry.chars().count() as u16;

        let mut spacing = " ".to_string();
        if raw_entry_width > width {
            let entry_overflow = raw_entry_width - width;
            label = label
                .chars()
                .take(label.chars().count() - entry_overflow as usize - 2)
                .collect::<String>()
                + "…";
        } else {
            let entry_underflow = width - raw_entry_width;
            spacing = " ".repeat(entry_underflow as usize);
        };
        let styled_label = if more_options {
            label
                .with(config.colors.more_options_color.into())
                .on(config.colors.bg_color.into())
        } else {
            label
                .with(config.colors.label_color.into())
                .on(config.colors.bg_color.into())
        };

        let styled_parts = vec![
            key.to_string()
                .with(config.colors.key_color.into())
                .on(config.colors.bg_color.into()),
            " → "
                .to_string()
                .with(config.colors.arrow_color.into())
                .on(config.colors.bg_color.into()),
            styled_label,
            spacing.on(config.colors.bg_color.into()),
            flags
                .with(config.colors.flags_color.into())
                .on(config.colors.bg_color.into()),
        ];
        Self {
            styled_parts,
        }
    }

    pub fn to_tty(&self, tty: &mut std::fs::File) -> std::io::Result<()> {
        for part in &self.styled_parts {
            write!(tty, "{}", part)?;
        }
        Ok(())
    }
}

fn format_flags(shortcut: &Shortcut, flag_symbols: &FlagSymbols) -> String {
    let mut flags: Vec<&str> = vec![];

    match shortcut.insert_type {
        InsertType::Replace => flags.push(&flag_symbols.replace),
        InsertType::Insert => flags.push(&flag_symbols.insert),
        InsertType::Append => flags.push(&flag_symbols.append),
        InsertType::Prepend => flags.push(&flag_symbols.prepend),
        InsertType::Surround => flags.push(&flag_symbols.surround),
    }

    if shortcut.evaluate {
        flags.push(&flag_symbols.evaluate);
    } else {
        flags.push(" ");
    }

    if shortcut.execute {
        flags.push(&flag_symbols.execute);
    } else {
        flags.push(" ");
    }

    flags.join(" ")
}
