use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

use crate::{ui::table, LeadrError};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InsertType {
    /// Replace the current prompt with the mapped command.
    #[default]
    Replace,

    /// Insert the mapped command at the current cursor position.
    Insert,

    /// Prepend the mapped command to the current prompt.
    Prepend,

    /// Append the mapped command to the current prompt.
    Append,

    /// Surround your prompt with a prefix and a suffix.
    Surround,
}

/// Represents a user-defined command with additional metadata.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Mapping {
    pub command: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this command should be executed automatically or just inserted.
    #[serde(default, skip_serializing_if = "is_replace")]
    pub insert_type: InsertType,

    /// Whether this command should be evaluated before being inserted.
    // default is false, skip serialization if false
    #[serde(default, skip_serializing_if = "is_false")]
    pub evaluate: bool,

    /// Whether this command should be executed immediately after being inserted.
    #[serde(default, skip_serializing_if = "is_false")]
    pub execute: bool,
}

impl Mapping {
    fn flags_string(&self) -> String {
        let mut flags = vec![format!("{:?}", self.insert_type).to_uppercase()];
        if self.evaluate {
            flags.push("EVAL".into());
        }
        if self.execute {
            flags.push("EXEC".into());
        }
        flags.join("+")
    }

    pub fn format_command(&self) -> String {
        format!("{} {}", self.flags_string(), self.command)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Mappings {
    mappings: HashMap<String, Mapping>,
}

impl Default for Mappings {
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
            // Surround with Quotes
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
        Self { mappings }
    }
}

impl Mappings {
    /// Loads mappings from a TOML file in the specified directory.
    pub fn load(config_dir: &Path) -> Result<Self, LeadrError> {
        let config_path = config_dir.join("mappings.toml");
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            let mappings: Mappings = toml::from_str(&contents)?;
            mappings.validate()?;
            Ok(mappings)
        } else {
            let mappings = Mappings::default();
            mappings.validate()?;
            Ok(mappings)
        }
    }

    /// Returns an exact match for a given sequence, if one exists.
    pub fn match_sequence(&self, sequence: &str) -> Option<&Mapping> {
        self.mappings.get(sequence)
    }

    /// Returns true if any mapping begins with the given sequence.
    pub fn has_partial_match(&self, seq: &str) -> bool {
        self.mappings.keys().any(|k| k.starts_with(seq))
    }

    pub fn grouped_next_options(&self, sequence: &str) -> HashMap<String, Vec<&Mapping>> {
        let mut grouped_options: HashMap<String, Vec<&Mapping>> = HashMap::new();

        for (key, mapping) in self.mappings.iter() {
            if key.starts_with(&sequence) {
                if let Some((_, char)) = key[sequence.len()..].char_indices().next() {
                    // next_key is this character (handle utf-8)
                    let next_key = char.to_string();
                    grouped_options.entry(next_key).or_default().push(mapping);
                }
            }
        }

        grouped_options
    }

    /// Validates that no mappings overlap or are prefixes of each other.
    fn validate(&self) -> Result<(), LeadrError> {
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

    /// Renders the configured mappings as a formatted table.
    pub fn render_table(&self) -> String {
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
}

fn is_replace(insert_type: &InsertType) -> bool {
    matches!(insert_type, InsertType::Replace)
}
fn is_false(b: &bool) -> bool {
    !*b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_replace_no_flags() {
        let sc = Mapping {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Replace,
            evaluate: false,
            execute: false,
        };
        assert_eq!(sc.format_command(), "REPLACE dummy command");
    }

    #[test]
    fn test_format_insert_eval_exec() {
        let sc = Mapping {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Insert,
            evaluate: true,
            execute: true,
        };
        assert_eq!(sc.format_command(), "INSERT+EVAL+EXEC dummy command");
    }

    #[test]
    fn test_format_append_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Append,
            evaluate: false,
            execute: false,
        };
        assert_eq!(sc.format_command(), "APPEND dummy command");
    }

    #[test]
    fn test_format_prepend_eval_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Prepend,
            evaluate: true,
            execute: false,
        };
        assert_eq!(sc.format_command(), "PREPEND+EVAL dummy command");
    }

    #[test]
    fn test_format_replace_exec_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Replace,
            evaluate: false,
            execute: true,
        };
        assert_eq!(sc.format_command(), "REPLACE+EXEC dummy command");
    }

    #[test]
    fn test_format_insert_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Insert,
            evaluate: false,
            execute: false,
        };
        assert_eq!(sc.format_command(), "INSERT dummy command");
    }

    #[test]
    fn test_format_surround() {
        let sc = Mapping {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Surround,
            evaluate: false,
            execute: true,
        };
        assert_eq!(sc.format_command(), "SURROUND+EXEC dummy command");
    }

    #[test]
    fn test_render_table_contains_mapping_keys() {
        let mappings = Mappings::default();
        let table = mappings.render_table();
        assert!(table.contains("gs"));
        assert!(table.contains("git status"));
        assert!(table.contains("Description"));
    }

    fn test_mappings() -> Mappings {
        let mut mappings = HashMap::new();
        mappings.insert(
            "gs".into(),
            Mapping {
                command: "git status".into(),
                description: None,
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: false,
            },
        );
        mappings.insert(
            "s".into(),
            Mapping {
                command: "sudo ".into(),
                description: None,
                insert_type: InsertType::Prepend,
                evaluate: false,
                execute: false,
            },
        );

        Mappings { mappings }
    }


    #[test]
    fn test_validate_mappings() {
        let mut mappings = test_mappings();
        assert!(mappings.validate().is_ok());

        mappings.mappings.insert(
            "g".into(),
            Mapping {
                command: "git".into(),
                description: Some("Git command".into()),
                insert_type: InsertType::Replace,
                evaluate: false,
                execute: true,
            },
        );

        // Validation should fail due to prefix conflict
        let result = mappings.validate();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(LeadrError::ConflictingSequenceError(_))
        ));
    }

    #[test]
    fn test_exact_match() {
        let mappings = test_mappings();

        let result = mappings.match_sequence("gs");
        assert!(result.is_some());
        assert_eq!(result.unwrap().insert_type, InsertType::Replace);

        let result = mappings.match_sequence("s");
        assert!(result.is_some());
        assert_eq!(result.unwrap().insert_type, InsertType::Prepend);

        let result = mappings.match_sequence("x");
        assert!(result.is_none());

        let result = mappings.match_sequence("g");
        assert!(result.is_none());
    }

    #[test]
    fn test_partial_match() {
        let mappings = test_mappings();

        assert!(mappings.has_partial_match("g"));
        assert!(!mappings.has_partial_match("x"));
    }
}
