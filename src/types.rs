use crate::EncodingStrings;

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ShortcutType {
    /// A shortcut that is executed immediately.
    #[default]
    Execute,

    /// A shortcut that is inserted into the command line, replacing existing text.
    Replace,

    /// A shortcut that is prepended to the currently typed command.
    Prepend,

    /// A shortcut that is appended to the currently typed command.
    Append,
}

/// Represents a user-defined command with additional metadata.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Shortcut {
    pub command: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this command should be executed automatically or just inserted.
    #[serde(default, skip_serializing_if = "is_execute")]
    pub shortcut_type: ShortcutType,
}

fn is_execute(shortcut_type: &ShortcutType) -> bool {
    matches!(shortcut_type, ShortcutType::Execute)
}

impl Shortcut {
    /// Formats the command, by injecting the encoding strings into the command.
    pub fn format_command(&self, encoding_strings: &EncodingStrings) -> String {
        match self.shortcut_type {
            ShortcutType::Execute => format!("{} {}", encoding_strings.exec_prefix, self.command),
            ShortcutType::Replace => self.command.to_string(),
            ShortcutType::Prepend => {
                format!("{} {}", encoding_strings.prepend_prefix, self.command)
            }
            ShortcutType::Append => format!("{} {}", encoding_strings.append_prefix, self.command),
        }
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
    fn test_format_command_exec() {
        let sc = Shortcut {
            command: "ls -la".into(),
            description: None,
            shortcut_type: ShortcutType::Execute,
        };
        let encoding_strings = EncodingStrings::default();
        assert_eq!(sc.format_command(&encoding_strings), "#EXEC ls -la");
    }

    #[test]
    fn test_format_command_replace() {
        let sc = Shortcut {
            command: "vim ".into(),
            description: Some("Edit file".into()),
            shortcut_type: ShortcutType::Replace,
        };
        let encoding_strings = EncodingStrings::default();
        assert_eq!(sc.format_command(&encoding_strings), "vim ");
    }

    #[test]
    fn test_format_command_prepend() {
        let sc = Shortcut {
            command: "sudo ".into(),
            description: None,
            shortcut_type: ShortcutType::Prepend,
        };
        let encoding_strings = EncodingStrings::default();
        assert_eq!(sc.format_command(&encoding_strings), "#PREPEND sudo ");
    }

    #[test]
    fn test_format_command_append() {
        let sc = Shortcut {
            command: "file.txt".into(),
            description: None,
            shortcut_type: ShortcutType::Append,
        };
        let encoding_strings = EncodingStrings::default();
        assert_eq!(sc.format_command(&encoding_strings), "#APPEND file.txt");
    }
}
