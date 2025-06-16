use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{LeadrError, ui::table};

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

/// Represents a user-defined key sequence to command mapping with additional metadata.
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

    #[serde(skip)]
    pub source_file: Option<std::path::PathBuf>,
}

impl Default for Mapping {
    fn default() -> Self {
        Mapping {
            command: String::new(),
            description: None,
            insert_type: InsertType::Replace,
            evaluate: false,
            execute: false,
            source_file: None,
        }
    }
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
                execute: true,
                ..Default::default()
            },
        );
        mappings.insert(
            "gc".into(),
            Mapping {
                command: "git commit -m \"#CURSOR\"".into(),
                description: Some("Start a Git commit".into()),
                ..Default::default()
            },
        );
        mappings.insert(
            "gs".into(),
            Mapping {
                command: "git status".into(),
                description: Some("Git status".into()),
                execute: true,
                ..Default::default()
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
                ..Default::default()
            },
        );
        mappings.insert(
            // Prepend Sudo
            "ps".into(),
            Mapping {
                command: "sudo ".into(),
                description: Some("Prepend sudo".into()),
                insert_type: InsertType::Prepend,
                ..Default::default()
            },
        );
        mappings.insert(
            // Surround with Quotes
            "sq".into(),
            Mapping {
                command: "\"#COMMAND\"".into(),
                description: Some("Surround with quotes".into()),
                insert_type: InsertType::Surround,
                ..Default::default()
            },
        );
        mappings.insert(
            // Yank to Clipboard
            "y".into(),
            Mapping {
                command: " | xclip -selection clipboard".into(),
                description: Some("Append copy to clipboard".into()),
                insert_type: InsertType::Append,
                ..Default::default()
            },
        );
        Self { mappings }
    }
}

impl Mappings {
    pub fn load(config_dir: &Path) -> Result<Self, LeadrError> {
        let mut merged = HashMap::new();

        // 1. Load main mappings.toml
        let main_file = config_dir.join("mappings.toml");
        if main_file.exists() {
            let contents = fs::read_to_string(&main_file)?;
            let mappings: Mappings = toml::from_str(&contents)?;
            for (key, mut mapping) in mappings.mappings {
                mapping.source_file = Some(main_file.clone());
                merged.insert(key, mapping);
            }
        }

        // 2. Load recursively from mappings/ directory
        let mappings_dir = config_dir.join("mappings");
        if mappings_dir.exists() && mappings_dir.is_dir() {
            for path in collect_toml_files(&mappings_dir)? {
                let contents = fs::read_to_string(&path)?;
                let mappings: Mappings = toml::from_str(&contents)?;
                for (key, mut mapping) in mappings.mappings {
                    mapping.source_file = Some(path.clone());
                    merged.insert(key, mapping);
                }
            }
        }

        let final_mappings = Mappings { mappings: merged };
        final_mappings.validate()?;
        Ok(final_mappings)
    }

    pub fn create_default(config_dir: &Path) -> Result<(), LeadrError> {
        std::fs::create_dir_all(config_dir)?;
        let mappings_path = config_dir.join("mappings.toml");
        if !mappings_path.exists() {
            let default_mappings = Mappings::default();
            let contents = toml::to_string(&default_mappings)?;
            std::fs::write(mappings_path, contents)?;
        }
        Ok(())
    }

    /// Returns an exact match for a given sequence, if one exists.
    pub fn match_sequence(&self, sequence: &str) -> Option<&Mapping> {
        self.mappings.get(sequence)
    }

    /// Returns true if any mapping begins with the given sequence.
    pub fn has_partial_match(&self, seq: &str) -> bool {
        self.mappings.keys().any(|k| k.starts_with(seq))
    }

    /// Given a sequence, checks all mappings that are still possible and groups them by the next character.
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

    fn validate(&self) -> Result<(), LeadrError> {
        let keys: Vec<&String> = self.mappings.keys().collect();

        // Validate that no mappings overlap or are prefixes of each other.
        for (i, key1) in keys.iter().enumerate() {
            for key2 in keys.iter().skip(i + 1) {
                if key1.starts_with(*key2) || key2.starts_with(*key1) {
                    let mapping1 = &self.mappings[*key1];
                    let mapping2 = &self.mappings[*key2];

                    let file1 = mapping1
                        .source_file
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "unknown source".to_string());

                    let file2 = mapping2
                        .source_file
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "unknown source".to_string());

                    return Err(LeadrError::ConflictingSequenceError(format!(
                        "'{}' (from {}) conflicts with '{}' (from {})",
                        key1, file1, key2, file2
                    )));
                }
            }
        }

        // Make sure that "Surround" type mappings contain "#COMMAND" in their command
        for mapping in self.mappings.values() {
            if mapping.insert_type == InsertType::Surround && !mapping.command.contains("#COMMAND")
            {
                let file = mapping
                    .source_file
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "unknown source".to_string());

                return Err(LeadrError::InvalidSurroundCommand(format!(
                    "Surround-type mapping '{}' (from {}) must contain '#COMMAND' in its command",
                    mapping.command, file
                )));
            }
        }

        Ok(())
    }

    pub fn render_table(&self) -> String {
        let layout = table::ColumnLayout {
            sequence: 8,
            command: 30,
            insert_type: 10,
            evaluate: 9,
            execute: 9,
            description: 40,
            source: 30,
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

fn collect_toml_files(dir: &Path) -> Result<Vec<PathBuf>, LeadrError> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            result.extend(collect_toml_files(&path)?);
        } else if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            result.push(path);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_replace_no_flags() {
        let sc = Mapping {
            command: "dummy command".into(),
            ..Default::default()
        };
        assert_eq!(sc.format_command(), "REPLACE dummy command");
    }

    #[test]
    fn test_format_insert_eval_exec() {
        let sc = Mapping {
            command: "dummy command".into(),
            insert_type: InsertType::Insert,
            evaluate: true,
            execute: true,
            ..Default::default()
        };
        assert_eq!(sc.format_command(), "INSERT+EVAL+EXEC dummy command");
    }

    #[test]
    fn test_format_append_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            insert_type: InsertType::Append,
            ..Default::default()
        };
        assert_eq!(sc.format_command(), "APPEND dummy command");
    }

    #[test]
    fn test_format_prepend_eval_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            insert_type: InsertType::Prepend,
            evaluate: true,
            ..Default::default()
        };
        assert_eq!(sc.format_command(), "PREPEND+EVAL dummy command");
    }

    #[test]
    fn test_format_replace_exec_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            insert_type: InsertType::Replace,
            execute: true,
            ..Default::default()
        };
        assert_eq!(sc.format_command(), "REPLACE+EXEC dummy command");
    }

    #[test]
    fn test_format_insert_only() {
        let sc = Mapping {
            command: "dummy command".into(),
            insert_type: InsertType::Insert,
            ..Default::default()
        };
        assert_eq!(sc.format_command(), "INSERT dummy command");
    }

    #[test]
    fn test_format_surround() {
        let sc = Mapping {
            command: "dummy command".into(),
            insert_type: InsertType::Surround,
            execute: true,
            ..Default::default()
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
                ..Default::default()
            },
        );
        mappings.insert(
            "s".into(),
            Mapping {
                command: "sudo ".into(),
                insert_type: InsertType::Prepend,
                ..Default::default()
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
                execute: true,
                ..Default::default()
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
