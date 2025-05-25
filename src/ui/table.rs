use crate::Shortcut;

pub struct ColumnLayout {
    pub sequence: usize,
    pub command: usize,
    pub insert_type: usize,
    pub evaluate: usize,
    pub execute: usize,
    pub description: usize,
}

pub fn render_header(layout: &ColumnLayout) -> String {
    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<eval$} {:<exec$} {:<desc$}\n",
        "Sequence",
        "Command",
        "Type",
        "Evaluate",
        "Execute",
        "Description",
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.insert_type,
        eval = layout.evaluate,
        exec = layout.execute,
        desc = layout.description,
    )
}

pub fn render_separator(layout: &ColumnLayout) -> String {
    format!(
        "{:-<seq$} {:-<cmd$} {:-<typ$} {:-<eval$} {:-<exec$} {:-<desc$}\n",
        "",
        "",
        "",
        "",
        "",
        "",
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.insert_type,
        eval = layout.evaluate,
        exec = layout.execute,
        desc = layout.description,
    )
}

fn truncate_string(cmd: &str, max_len: usize) -> String {
    if cmd.chars().count() > max_len {
        cmd.chars().take(max_len.saturating_sub(3)).collect::<String>() + "..."
    } else {
        cmd.to_string()
    }
}

pub fn render_row(layout: &ColumnLayout, sequence: &str, shortcut: &Shortcut) -> String {
    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<eval$} {:<exec$} {:<desc$}\n",
        sequence,
        truncate_string(&shortcut.command, layout.command),
        format!("{:?}", shortcut.insert_type),
        if shortcut.evaluate { "Yes" } else { "No" },
        if shortcut.execute { "Yes" } else { "No" },
        truncate_string(&shortcut.description.clone().unwrap_or_default(), layout.description),
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.insert_type,
        eval = layout.evaluate,
        exec = layout.execute,
        desc = layout.description,
    )
}

