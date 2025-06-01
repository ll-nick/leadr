use serde::{Deserialize, Serialize};

use crate::types::{InsertType, Shortcut};

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
pub struct Config {
    pub flag_symbols: FlagSymbols,
}

pub struct Entry {
    key: String,
    label: String,
    flags: String,
    width: u16,
    config: Config,
}

impl Entry {
    pub fn new(key: &str, shortcuts: &Vec<&Shortcut>, width: u16, config: Config) -> Self {
        let label = if shortcuts.len() != 1 {
            format!("+{} options", shortcuts.len())
        } else {
            let shortcut = &shortcuts.first().unwrap();
            shortcut
                .description
                .as_deref()
                .unwrap_or(&shortcut.command)
                .to_string()
        };

        let flags = if shortcuts.len() != 1 {
            " ".repeat(5) // make sure to take up space that flags would take
        } else {
            let shortcut = &shortcuts.first().unwrap();
            format_flags(shortcut, &config.flag_symbols)
        };

        Self {
            key: key.to_string(),
            label,
            flags,
            width,
            config,
        }
    }

    fn format_entry_raw(&self) -> String {
        format!("{} → {} {}", self.key, self.label, self.flags)
    }

    fn format_entry(&self) -> String {
        let raw_entry_width = self.format_entry_raw().chars().count() as u16;
        let mut label = self.label.clone();
        let mut spacing = "".to_string();
        if raw_entry_width > self.width {
            let entry_overflow = raw_entry_width - self.width;
            label = label
                .chars()
                .take(self.label.chars().count() - entry_overflow as usize - 1)
                .collect::<String>()
                + "…";
        } else {
            let entry_underflow = self.width - raw_entry_width;
            spacing = " ".repeat(entry_underflow as usize);
        };

        format!("{} → {} {}{}", self.key, spacing, label, self.flags)
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_entry())
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
