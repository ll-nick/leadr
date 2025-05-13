#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Shortcut {
    pub command: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default="default_execute", skip_serializing_if = "is_default_execute")]
    pub execute: bool,
}

fn default_execute() -> bool {
    true
}
fn is_default_execute(val: &bool) -> bool {
    *val
}

impl Shortcut {
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
