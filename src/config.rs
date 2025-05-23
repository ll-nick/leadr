use std::collections::HashMap;

use crate::types::Shortcut;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncodingStrings {
    pub cursor_position: String,
    pub exec_prefix: String,
}

impl std::default::Default for EncodingStrings {
    fn default() -> Self {
        Self {
            cursor_position: "#CURSOR".into(),
            exec_prefix: "#EXEC".into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// String that will be injected into the command, the interpreted by the shell script.
    /// These usually do not need to be changed unless they conflict with a shell command.
    #[serde(skip_serializing)]
    pub encoding_strings: EncodingStrings,

    /// The key binding to activate the shortcut handler.
    #[serde(default = "default_leadr_key")]
    pub leadr_key: String,

    /// Whether or not to print the sequence of keys pressed at the bottom of the screen.
    #[serde(default = "default_print_sequence")]
    pub print_sequence: bool,

    /// Padding from the right edge of the screen when rendering sequences.
    #[serde(
        default = "default_padding",
        skip_serializing_if = "is_default_padding"
    )]
    pub padding: usize,

    /// The shortcut mappings from key sequences to commands.
    pub shortcuts: HashMap<String, Shortcut>,
}

impl Config {
    /// Renders the configured shortcuts as a formatted table.
    pub fn render_shortcut_table(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "{:<8} {:<30} {}\n",
            "Keys", "Command", "Description"
        ));
        output.push_str(&format!("{:-<8} {:-<30} {:-<}\n", "", "", ""));
        for (key, shortcut) in &self.shortcuts {
            output.push_str(&format!(
                "{:<8} {:<30} {}\n",
                key,
                shortcut.command,
                shortcut.description.clone().unwrap_or_default()
            ));
        }
        output
    }
}

fn default_leadr_key() -> String {
    "<C-Space>".into()
}

fn default_exec_prefix() -> String {
    "#EXEC".into()
}
fn is_default_exec_prefix(val: &str) -> bool {
    val == default_exec_prefix()
}

fn default_print_sequence() -> bool {
    false
}

fn default_padding() -> usize {
    4
}
fn is_default_padding(val: &usize) -> bool {
    *val == default_padding()
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        let mut shortcuts = HashMap::new();
        shortcuts.insert(
            "gs".into(),
            Shortcut {
                command: "git status".into(),
                description: Some("Git status".into()),
                execute: true,
            },
        );
        shortcuts.insert(
            "ga".into(),
            Shortcut {
                command: "git add .".into(),
                description: Some("Git add all".into()),
                execute: true,
            },
        );
        shortcuts.insert(
            "gc".into(),
            Shortcut {
                command: "git commit -m \"".into(),
                description: Some("Start a Git commit".into()),
                execute: false,
            },
        );
        shortcuts.insert(
            "gp".into(),
            Shortcut {
                command: "git push".into(),
                description: Some("Git push".into()),
                execute: true,
            },
        );
        shortcuts.insert(
            "gl".into(),
            Shortcut {
                command: "git log --oneline".into(),
                description: Some("Compact Git log".into()),
                execute: true,
            },
        );
        shortcuts.insert(
            "h".into(),
            Shortcut {
                command: "htop".into(),
                description: Some("System monitor".into()),
                execute: true,
            },
        );
        shortcuts.insert(
            "ip".into(),
            Shortcut {
                command: "ip addr show".into(),
                description: Some("Show IP addresses".into()),
                execute: true,
            },
        );
        Self {
            leadr_key: default_leadr_key(),
            exec_prefix: default_exec_prefix(),
            print_sequence: default_print_sequence(),
            padding: default_padding(),
            encoding_strings: EncodingStrings::default(),
            shortcuts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.leadr_key, "<C-Space>");
        assert_eq!(config.encoding_strings.exec_prefix, "#EXEC");
        assert_eq!(config.encoding_strings.cursor_position, "#CURSOR");
        assert!(!config.print_sequence);
        assert_eq!(config.padding, 4);
        assert!(config.shortcuts.contains_key("gs"));
    }

    #[test]
    fn test_render_table_contains_shortcut_keys() {
        let config = Config::default();
        let table = config.render_shortcut_table();
        assert!(table.contains("gs"));
        assert!(table.contains("git status"));
        assert!(table.contains("Description"));
    }
}
