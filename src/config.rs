use std::{collections::HashMap, time::Duration};

use crate::{
    types::{InsertType, Shortcut},
    ui::{overlay::Config as OverlayConfig, table},
    LeadrError,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// The key binding to activate the shortcut handler.
    pub leadr_key: String,

    /// Whether or not to print the ui overlay.
    pub show_overlay: bool,

    /// The duration until the overlay appears.
    pub overlay_timeout: Duration,

    /// The overlay styling.
    #[serde(skip_serializing)]
    pub overlay_style: OverlayConfig,

    /// The shortcut mappings from key sequences to commands.
    pub shortcuts: HashMap<String, Shortcut>,
}

impl Default for Config {
    fn default() -> Self {
        let mut shortcuts = HashMap::new();
        shortcuts.insert(
            "ga".into(),
            Shortcut {
                command: "git add .".into(),
                description: Some("Git add all".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        shortcuts.insert(
            "gc".into(),
            Shortcut {
                command: "git commit -m \"#CURSOR\"".into(),
                description: Some("Start a Git commit".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: false,
            },
        );
        shortcuts.insert(
            "gs".into(),
            Shortcut {
                command: "git status".into(),
                description: Some("Git status".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        shortcuts.insert(
            // Insert Date
            "id".into(),
            Shortcut {
                command: "date +%Y%m%d".into(),
                description: Some("Insert current date in YYYYMMDD format".into()),
                insert_type: InsertType::Insert,
                evaluate: true,
                execute: false,
            },
        );
        shortcuts.insert(
            // Prepend Sudo
            "ps".into(),
            Shortcut {
                command: "sudo ".into(),
                description: Some("Prepend sudo".into()),
                insert_type: InsertType::Prepend,
                evaluate: false,
                execute: false,
            },
        );
        shortcuts.insert(
            // Substitute Command
            "sq".into(),
            Shortcut {
                command: "\"#COMMAND\"".into(),
                description: Some("Surround with quotes".into()),
                insert_type: InsertType::Surround,
                evaluate: false,
                execute: false,
            },
        );
        shortcuts.insert(
            // Yank to Clipboard
            "y".into(),
            Shortcut {
                command: " | xclip -selection clipboard".into(),
                description: Some("Append copy to clipboard".into()),
                insert_type: InsertType::Append,
                evaluate: false,
                execute: false,
            },
        );
        Self {
            leadr_key: "<C-g>".into(),
            show_overlay: true,
            overlay_timeout: Duration::from_millis(500),
            overlay_style: OverlayConfig::default(),
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
            insert_type: 10,
            evaluate: 9,
            execute: 9,
            description: 40,
        };

        let mut table = String::new();
        table.push_str(&table::render_header(&layout));
        table.push_str(&table::render_separator(&layout));

        let mut keys: Vec<_> = self.shortcuts.keys().collect();
        keys.sort(); // Sorts alphabetically (lexicographically)

        for key in keys {
            let shortcut = &self.shortcuts[key];
            table.push_str(&table::render_row(&layout, key, shortcut));
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

        // Make sure that "Surround" type shortcuts contain "#COMMAND" in their command
        for shortcut in self.shortcuts.values() {
            if shortcut.insert_type == InsertType::Surround
                && !shortcut.command.contains("#COMMAND")
            {
                return Err(LeadrError::InvalidSurroundCommand(format!(
                    "Shortcut '{}' must contain '#COMMAND' in its command",
                    shortcut.command
                )));
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
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        shortcuts.insert(
            "gs".into(),
            Shortcut {
                command: "git status".into(),
                description: Some("Git status".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );

        let config = Config {
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
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        shortcuts.insert(
            "x".into(),
            Shortcut {
                command: "exit".into(),
                description: Some("Exit command".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );

        let config = Config {
            leadr_key: "<C-g>".into(),
            print_sequence: false,
            padding: 4,
            shortcuts,
        };

        // Validation should succeed with no conflicts
        assert!(config.validate().is_ok());
    }
}
