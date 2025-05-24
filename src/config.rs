use std::collections::HashMap;

use crate::error::LeadrError;
use crate::ui::table;
use crate::types::{Shortcut, ShortcutType};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncodingStrings {
    pub cursor_position: String,
    pub exec_prefix: String,
    pub append_prefix: String,
    pub prepend_prefix: String,
}

impl Default for EncodingStrings {
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

impl Default for Config {
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
            "s".into(),
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

impl Config {
    /// Renders the configured shortcuts as a formatted table.
    pub fn render_shortcut_table(&self) -> String {
        let layout = table::ColumnLayout {
            sequence: 8,
            command: 30,
            shortcut_type: 15,
            description: 40,
        };

        let mut table = String::new();
        table.push_str(&table::render_header(&layout));
        table.push_str(&table::render_separator(&layout));

        for (sequence, shortcut) in &self.shortcuts {
            table.push_str(&table::render_row(&layout, sequence, shortcut));
        }

        table
    }

    /// Validates that no shortcuts overlap or are prefixes of each other.
    pub fn validate(&self) -> Result<(), LeadrError> {
        let keys: Vec<&String> = self.shortcuts.keys().collect();

        for (i, key1) in keys.iter().enumerate() {
            for key2 in keys.iter().skip(i + 1) {
                if key1.starts_with(*key2) || key2.starts_with(*key1) {
                    return Err(LeadrError::ConflictingSequenceError(format!(
                        "'{}' conflicts with '{}'",
                        key1, key2
                    )));
                }
            }
        }
        Ok(())
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

    #[test]
    fn test_validate_shortcuts() {
        // Create a config with conflicting shortcuts: "g" and "gs"
        let mut shortcuts = HashMap::new();
        shortcuts.insert(
            "g".into(),
            Shortcut {
                command: "git".into(),
                description: Some("Git command".into()),
                shortcut_type: ShortcutType::Execute,
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

        let config = Config {
            encoding_strings: EncodingStrings::default(),
            leadr_key: "<C-g>".into(),
            print_sequence: false,
            padding: 4,
            shortcuts,
        };

        // Validation should fail due to prefix conflict
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(LeadrError::ConflictingSequenceError(_))
        ));

        // Now create a config with no conflicts
        let mut shortcuts = HashMap::new();
        shortcuts.insert(
            "g".into(),
            Shortcut {
                command: "git".into(),
                description: Some("Git command".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );
        shortcuts.insert(
            "x".into(),
            Shortcut {
                command: "exit".into(),
                description: Some("Exit command".into()),
                shortcut_type: ShortcutType::Execute,
            },
        );

        let config = Config {
            encoding_strings: EncodingStrings::default(),
            leadr_key: "<C-g>".into(),
            print_sequence: false,
            padding: 4,
            shortcuts,
        };

        // Validation should succeed with no conflicts
        assert!(config.validate().is_ok());
    }
}
