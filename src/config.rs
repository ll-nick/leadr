use crate::types::Shortcut;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_leader")]
    pub leader: String,
    #[serde(default = "default_exec_prefix")]
    pub exec_prefix: String,
    pub shortcuts: Vec<Shortcut>,
}

impl Config {
    pub fn render_table(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "{:<8} {:<30} {}\n",
            "Keys", "Command", "Description"
        ));
        output.push_str(&format!("{:-<8} {:-<30} {:-<}\n", "", "", ""));
        for shortcut in &self.shortcuts {
            output.push_str(&format!(
                "{:<8} {:<30} {}\n",
                shortcut.sequence,
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

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            leader: default_leader(),
            exec_prefix: default_exec_prefix(),
            shortcuts: vec![
                // File navigation
                Shortcut {
                    sequence: "ll".into(),
                    command: "ls -la".into(),
                    description: Some("List directory contents (detailed)".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "..".into(),
                    command: "cd ..".into(),
                    description: Some("Go up one directory".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "cc".into(),
                    command: "cd ~".into(),
                    description: Some("Change to home directory".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: ".".into(),
                    command: "source .".into(),
                    description: Some("Source local environment file".into()),
                    execute: true,
                },
                // Git
                Shortcut {
                    sequence: "gs".into(),
                    command: "git status".into(),
                    description: Some("Git status".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "ga".into(),
                    command: "git add .".into(),
                    description: Some("Git add all".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "gc".into(),
                    command: "git commit -m \"".into(),
                    description: Some("Start a Git commit".into()),
                    execute: false,
                },
                Shortcut {
                    sequence: "gp".into(),
                    command: "git push".into(),
                    description: Some("Git push".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "gl".into(),
                    command: "git log --oneline".into(),
                    description: Some("Compact Git log".into()),
                    execute: true,
                },
                // System utilities
                Shortcut {
                    sequence: "rm".into(),
                    command: "rm -r ".into(),
                    description: Some("Remove file".into()),
                    execute: false,
                },
                Shortcut {
                    sequence: "h".into(),
                    command: "htop".into(),
                    description: Some("System monitor".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "p".into(),
                    command: "ping google.com".into(),
                    description: Some("Ping Google".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "df".into(),
                    command: "df -h".into(),
                    description: Some("Disk usage".into()),
                    execute: true,
                },
                // Networking
                Shortcut {
                    sequence: "ip".into(),
                    command: "ip a".into(),
                    description: Some("Show IP addresses".into()),
                    execute: true,
                },
                Shortcut {
                    sequence: "ss".into(),
                    command: "ss -tuln".into(),
                    description: Some("Show open sockets and ports".into()),
                    execute: true,
                },
            ],
        }
    }
}
