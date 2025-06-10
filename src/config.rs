use std::{collections::HashMap, path::Path, time::Duration};

use crate::{
    types::{InsertType, Mapping, Mappings},
    ui::{overlay::Config as OverlayConfig, table},
    LeadrError,
};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// The key binding to activate leadr.
    pub leadr_key: String,

    /// Whether or not to print the ui overlay.
    pub show_overlay: bool,

    /// The duration until the overlay appears.
    pub overlay_timeout: Duration,

    /// The overlay styling.
    pub overlay_style: OverlayConfig,

    /// The mappings from key sequences to commands.
    pub mappings: Mappings,
}

impl Default for Config {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(
            "ga".into(),
            Mapping {
                command: "git add .".into(),
                description: Some("Git add all".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        mappings.insert(
            "gc".into(),
            Mapping {
                command: "git commit -m \"#CURSOR\"".into(),
                description: Some("Start a Git commit".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: false,
            },
        );
        mappings.insert(
            "gs".into(),
            Mapping {
                command: "git status".into(),
                description: Some("Git status".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        mappings.insert(
            // Insert Date
            "id".into(),
            Mapping {
                command: "date +%Y%m%d".into(),
                description: Some("Insert current date in YYYYMMDD format".into()),
                insert_type: InsertType::Insert,
                evaluate: true,
                execute: false,
            },
        );
        mappings.insert(
            // Prepend Sudo
            "ps".into(),
            Mapping {
                command: "sudo ".into(),
                description: Some("Prepend sudo".into()),
                insert_type: InsertType::Prepend,
                evaluate: false,
                execute: false,
            },
        );
        mappings.insert(
            // Substitute Command
            "sq".into(),
            Mapping {
                command: "\"#COMMAND\"".into(),
                description: Some("Surround with quotes".into()),
                insert_type: InsertType::Surround,
                evaluate: false,
                execute: false,
            },
        );
        mappings.insert(
            // Yank to Clipboard
            "y".into(),
            Mapping {
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
            mappings,
        }
    }
}

impl Config {
    pub fn load(config_dir: &Path) -> Result<Self, LeadrError> {
        let config_path = config_dir.join("config.toml");
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            let config = toml::from_str(&contents)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Renders the configured mappings as a formatted table.
    pub fn render_mapping_table(&self) -> String {
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

        let mut keys: Vec<_> = self.mappings.keys().collect();
        keys.sort(); // Sorts alphabetically (lexicographically)

        for key in keys {
            let mapping = &self.mappings[key];
            table.push_str(&table::render_row(&layout, key, mapping));
        }

        table
    }

    /// Validates that no mappings overlap or are prefixes of each other.
    pub fn validate(&self) -> Result<(), LeadrError> {
        let keys: Vec<&String> = self.mappings.keys().collect();

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

        // Make sure that "Surround" type mappings contain "#COMMAND" in their command
        for mapping in self.mappings.values() {
            if mapping.insert_type == InsertType::Surround && !mapping.command.contains("#COMMAND")
            {
                return Err(LeadrError::InvalidSurroundCommand(format!(
                    "Surround-type mapping '{}' must contain '#COMMAND' in its command",
                    mapping.command
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
    fn test_render_table_contains_mapping_keys() {
        let config = Config::default();
        let table = config.render_mapping_table();
        assert!(table.contains("gs"));
        assert!(table.contains("git status"));
        assert!(table.contains("Description"));
    }

    #[test]
    fn test_validate_mappings() {
        // Create a config with conflicting mappings: "g" and "gs"
        let mut mappings = HashMap::new();
        mappings.insert(
            "g".into(),
            Mapping {
                command: "git".into(),
                description: Some("Git command".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        mappings.insert(
            "gs".into(),
            Mapping {
                command: "git status".into(),
                description: Some("Git status".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );

        let config = Config {
            mappings,
            ..Default::default()
        };

        // Validation should fail due to prefix conflict
        let result = config.validate();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(LeadrError::ConflictingSequenceError(_))
        ));

        // Now create a config with no conflicts
        let mut mappings = HashMap::new();
        mappings.insert(
            "g".into(),
            Mapping {
                command: "git".into(),
                description: Some("Git command".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );
        mappings.insert(
            "x".into(),
            Mapping {
                command: "exit".into(),
                description: Some("Exit command".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );

        let config = Config {
            mappings,
            ..Default::default()
        };

        // Validation should succeed with no conflicts
        assert!(config.validate().is_ok());
    }
}
