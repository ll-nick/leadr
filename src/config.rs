use std::collections::HashMap;

use crate::types::Shortcut;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_leader")]
    pub leader: String,
    #[serde(default = "default_exec_prefix")]
    pub exec_prefix: String,
    #[serde(default = "default_padding")]
    pub padding: usize,
    pub shortcuts: HashMap<String, Shortcut>,
}

impl Config {
    pub fn render_table(&self) -> String {
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

fn default_leader() -> String {
    "<C-Space>".into()
}

fn default_exec_prefix() -> String {
    "#EXEC".into()
}

fn default_padding() -> usize {
    4
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        let mut shortcuts = HashMap::new();
        shortcuts.insert("gs".into(), Shortcut {
            command: "git status".into(),
            description: Some("Git status".into()),
            execute: true,
        });
        shortcuts.insert("ga".into(), Shortcut {
            command: "git add .".into(),
            description: Some("Git add all".into()),
            execute: true,
        });
        shortcuts.insert("gc".into(), Shortcut {
            command: "git commit -m \"".into(),
            description: Some("Start a Git commit".into()),
            execute: false,
        });
        shortcuts.insert("gp".into(), Shortcut {
            command: "git push".into(),
            description: Some("Git push".into()),
            execute: true,
        });
        shortcuts.insert("gl".into(), Shortcut {
            command: "git log --oneline".into(),
            description: Some("Compact Git log".into()),
            execute: true,
        });
        shortcuts.insert("h".into(), Shortcut {
            command: "htop".into(),
            description: Some("System monitor".into()),
            execute: true,
        });
        shortcuts.insert("ip".into(), Shortcut {
            command: "ip addr show".into(),
            description: Some("Show IP addresses".into()),
            execute: true,
        });
        Self {
            leader: default_leader(),
            exec_prefix: default_exec_prefix(),
            padding: default_padding(),
            shortcuts,
        }
    }
}
