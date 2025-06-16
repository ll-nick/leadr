use crate::Mapping;

pub struct ColumnLayout {
    pub sequence: usize,
    pub command: usize,
    pub insert_type: usize,
    pub evaluate: usize,
    pub execute: usize,
    pub description: usize,
    pub source: usize,
}

pub fn render_header(layout: &ColumnLayout) -> String {
    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<eval$} {:<exec$} {:<desc$} {:<src$}\n",
        "Sequence",
        "Command",
        "Type",
        "Evaluate",
        "Execute",
        "Description",
        "Source",
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.insert_type,
        eval = layout.evaluate,
        exec = layout.execute,
        desc = layout.description,
        src = layout.source,
    )
}

pub fn render_separator(layout: &ColumnLayout) -> String {
    format!(
        "{:-<seq$} {:-<cmd$} {:-<typ$} {:-<eval$} {:-<exec$} {:-<desc$} {:-<src$}\n",
        "",
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
        src = layout.source,
    )
}

fn truncate_string(cmd: &str, max_len: usize) -> String {
    if cmd.chars().count() > max_len {
        cmd.chars()
            .take(max_len.saturating_sub(3))
            .collect::<String>()
            + "..."
    } else {
        cmd.to_string()
    }
}

pub fn render_row(layout: &ColumnLayout, sequence: &str, mapping: &Mapping) -> String {
    let source = mapping
        .source_file
        .as_ref()
        .map(|p| {
            let path_str = p.display().to_string();
            if let Some(pos) = path_str.find("mappings/") {
                path_str[pos..].to_string()
            } else if path_str.ends_with("mappings.toml") {
                "mappings.toml".to_string()
            } else {
                path_str
            }
        })
        .unwrap_or_default()
        .to_string();

    format!(
        "{:<seq$} {:<cmd$} {:<typ$} {:<eval$} {:<exec$} {:<desc$} {:<src$}\n",
        sequence,
        truncate_string(&mapping.command, layout.command),
        format!("{:?}", mapping.insert_type),
        if mapping.evaluate { "Yes" } else { "No" },
        if mapping.execute { "Yes" } else { "No" },
        truncate_string(
            &mapping.description.clone().unwrap_or_default(),
            layout.description
        ),
        truncate_string(&source, layout.source),
        seq = layout.sequence,
        cmd = layout.command,
        typ = layout.insert_type,
        eval = layout.evaluate,
        exec = layout.execute,
        desc = layout.description,
        src = layout.source,
    )
}
