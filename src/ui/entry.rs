use std::io::Write;

use crossterm::style::Stylize;

use crate::{InsertType, Mapping, Symbols, Theme, mappings::MatchType};

pub struct Entry {
    pub styled_parts: Vec<crossterm::style::StyledContent<String>>,
}

impl Entry {
    pub fn new(
        key: &str,
        match_type: MatchType,
        width: u16,
        symbols: &Symbols,
        theme: &Theme,
    ) -> Self {
        let (mut label, flags, is_prefix) = match match_type {
            MatchType::Exact(mapping) => {
                let label = mapping
                    .description
                    .as_deref()
                    .unwrap_or(&mapping.command)
                    .to_string();
                let flags = format_flags(mapping, symbols);
                (label, flags, false)
            }
            MatchType::Prefix(count) => (format!("+{} mappings", count), " ".repeat(5), true),
            MatchType::None => ("(invalid)".into(), "".into(), true),
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
        let styled_label = if is_prefix {
            label
                .with(theme.text_primary.into())
                .on(theme.background.into())
        } else {
            label
                .with(theme.text_highlight_secondary.into())
                .on(theme.background.into())
        };

        let styled_parts = vec![
            key.to_string()
                .with(theme.text_primary.into())
                .on(theme.background.into()),
            format!(" {} ", symbols.arrow)
                .to_string()
                .with(theme.text_secondary.into())
                .on(theme.background.into()),
            styled_label,
            spacing.on(theme.background.into()),
            flags
                .with(theme.text_highlight_primary.into())
                .on(theme.background.into()),
        ];
        Self { styled_parts }
    }

    pub fn to_tty(&self, tty: &mut std::fs::File) -> std::io::Result<()> {
        for part in &self.styled_parts {
            write!(tty, "{}", part)?;
        }
        Ok(())
    }
}

fn format_flags(mapping: &Mapping, symbols: &Symbols) -> String {
    let mut flags: Vec<&str> = vec![];

    match mapping.insert_type {
        InsertType::Replace => flags.push(&symbols.replace),
        InsertType::Insert => flags.push(&symbols.insert),
        InsertType::Append => flags.push(&symbols.append),
        InsertType::Prepend => flags.push(&symbols.prepend),
        InsertType::Surround => flags.push(&symbols.surround),
    }

    if mapping.evaluate {
        flags.push(&symbols.evaluate);
    } else {
        flags.push(" ");
    }

    if mapping.execute {
        flags.push(&symbols.execute);
    } else {
        flags.push(" ");
    }

    flags.join(" ")
}
