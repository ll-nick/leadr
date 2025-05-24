use crate::Shortcut;

pub struct ColumnLayout {
    pub sequence: usize,
    pub command: usize,
    pub shortcut_type: usize,
    pub description: usize,
}

pub fn render_header(layout: &ColumnLayout) -> String {
    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<desc$}\n",
        "Sequence",
        "Command",
        "Type",
        "Description",
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.shortcut_type,
        desc = layout.description,
    )
}

pub fn render_separator(layout: &ColumnLayout) -> String {
    format!(
        "{:-<seq$} {:-<cmd$} {:-<typ$} {:-<desc$}\n",
        "",
        "",
        "",
        "",
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.shortcut_type,
        desc = layout.description,
    )
}

pub fn render_row(layout: &ColumnLayout, sequence: &str, shortcut: &Shortcut) -> String {
    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<desc$}\n",
        sequence,
        shortcut.command,
        format!("{:?}", shortcut.shortcut_type),
        shortcut.description.clone().unwrap_or_default(),
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.shortcut_type,
        desc = layout.description,
    )
}

