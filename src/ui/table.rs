use crate::Shortcut;

pub struct ColumnLayout {
    pub sequence: usize,
    pub command: usize,
    pub insert_type: usize,
    pub eval: usize,
    pub execute: usize,
    pub description: usize,
}

pub fn render_header(layout: &ColumnLayout) -> String {
    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<eval$} {:<exec$} {:<desc$}\n",
        "Sequence",
        "Command",
        "Type",
        "Eval",
        "Exec",
        "Description",
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.insert_type,
        eval = layout.eval,
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
        eval = layout.eval,
        exec = layout.execute,
        desc = layout.description,
    )
}

pub fn render_row(layout: &ColumnLayout, sequence: &str, shortcut: &Shortcut) -> String {
    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<eval$} {:<exec$} {:<desc$}\n",
        sequence,
        shortcut.command,
        format!("{:?}", shortcut.insert_type),
        if shortcut.eval { "Yes" } else { "No" },
        if shortcut.execute { "Yes" } else { "No" },
        shortcut.description.clone().unwrap_or_default(),
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.insert_type,
        eval = layout.eval,
        exec = layout.execute,
        desc = layout.description,
    )
}

