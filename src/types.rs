/// Represents a user-defined command with additional metadata.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Shortcut {
    pub command: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this command should be executed automatically or just inserted.
    #[serde(
        default = "default_execute",
        skip_serializing_if = "is_default_execute"
    )]
    pub execute: bool,
}

fn default_execute() -> bool {
    true
}
fn is_default_execute(val: &bool) -> bool {
    *val
}

impl Shortcut {
    /// Formats the command, by applying the exec prefix if applicable.
    pub fn format_command(&self, exec_prefix: &str) -> String {
        if self.execute {
            format!("{} {}", exec_prefix, self.command)
        } else {
            self.command.to_string()
        }
    }
}

pub enum ShortcutResult {
    Shortcut(Shortcut),
    Cancelled,
    NoMatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_command_exec_true() {
        let sc = Shortcut {
            command: "ls -la".into(),
            description: None,
            execute: true,
        };
        assert_eq!(sc.format_command("#EXEC"), "#EXEC ls -la");
    }

    #[test]
    fn test_format_command_exec_false() {
        let sc = Shortcut {
            command: "vim ".into(),
            description: Some("Edit file".into()),
            execute: false,
        };
        assert_eq!(sc.format_command("#EXEC"), "vim ");
    }
}
