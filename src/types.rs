use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InsertType {
    /// A shortcut that is inserted into the command line, replacing existing text.
    #[default]
    Replace,

    /// A shortcut that is inserted into the command line at the current cursor position.
    Insert,

    /// A shortcut that is prepended to the currently typed command.
    Prepend,

    /// A shortcut that is appended to the currently typed command.
    Append,

    /// Add a string before and after the currently typed command.
    Surround,
}

/// Represents a user-defined command with additional metadata.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Shortcut {
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

pub type Shortcuts = HashMap<String, Shortcut>;

fn is_replace(insert_type: &InsertType) -> bool {
    matches!(insert_type, InsertType::Replace)
}
fn is_false(b: &bool) -> bool {
    !*b
}

impl Shortcut {
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

pub enum ShortcutResult {
    Shortcut(String),
    Cancelled,
    NoMatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_replace_no_flags() {
        let sc = Shortcut {
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
        let sc = Shortcut {
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
        let sc = Shortcut {
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
        let sc = Shortcut {
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
        let sc = Shortcut {
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
        let sc = Shortcut {
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
        let sc = Shortcut {
            command: "dummy command".into(),
            description: None,
            insert_type: InsertType::Surround,
            evaluate: false,
            execute: true,
        };
        assert_eq!(sc.format_command(), "SURROUND+EXEC dummy command");
    }
}
