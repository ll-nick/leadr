use std::collections::HashMap;

use crate::types::{Shortcut, ShortcutType};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncodingStrings {
    pub cursor_position: String,
    pub exec_prefix: String,
    pub append_prefix: String,
    pub prepend_prefix: String,
}

impl std::default::Default for EncodingStrings {
    fn default() -> Self {
        Self {
            cursor_position: "#CURSOR".into(),
            exec_prefix: "#EXEC".into(),
            append_prefix: "#APPEND".into(),
            prepend_prefix: "#PREPEND".into(),
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
    pub leadr_key: String,

    /// Whether or not to print the sequence of keys pressed at the bottom of the screen.
    pub print_sequence: bool,

    /// Padding from the right edge of the screen when rendering sequences.
    #[serde(skip_serializing)]
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

impl ::std::default::Default for Config {
    fn default() -> Self {
        let mut shortcuts = HashMap::new();
        shortcuts.insert(
            "c".into(),
            Shortcut {
                command: " | xclip -selection clipboard".into(),
                description: Some("Append copy to clipboard".into()),
                shortcut_type: ShortcutType::Append,
            },
        );
        shortcuts.insert(
            "gs".into(),
            Shortcut {
                command: "git status".into(),
                description: Some("Git status".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );
        shortcuts.insert(
            "ga".into(),
            Shortcut {
                command: "git add .".into(),
                description: Some("Git add all".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );
        shortcuts.insert(
            "gc".into(),
            Shortcut {
                command: "git commit -m \"#CURSOR\"".into(),
                description: Some("Start a Git commit".into()),
                shortcut_type: ShortcutType::Replace,
            },
        );
        shortcuts.insert(
            "gp".into(),
            Shortcut {
                command: "git push".into(),
                description: Some("Git push".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );
        shortcuts.insert(
            "gl".into(),
            Shortcut {
                command: "git log --oneline".into(),
                description: Some("Compact Git log".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );
        shortcuts.insert(
            "h".into(),
            Shortcut {
                command: "htop".into(),
                description: Some("System monitor".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );
        shortcuts.insert(
            "ip".into(),
            Shortcut {
                command: "ip addr show".into(),
                description: Some("Show IP addresses".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );
        shortcuts.insert(
            "ps".into(),
            Shortcut {
                command: "sudo ".into(),
                description: Some("Prepend sudo".into()),
                shortcut_type: ShortcutType::Prepend,
            },
        );
        Self {
            encoding_strings: EncodingStrings::default(),
            leadr_key: "<C-g>".into(),
            print_sequence: false,
            padding: 4,
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
        assert_eq!(config.leadr_key, "<C-g>");
        assert_eq!(config.encoding_strings.exec_prefix, "#EXEC");
        assert_eq!(config.encoding_strings.cursor_position, "#CURSOR");
        assert_eq!(config.encoding_strings.append_prefix, "#APPEND");
        assert_eq!(config.encoding_strings.prepend_prefix, "#PREPEND");
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
