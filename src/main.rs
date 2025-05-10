use leadr::{Shortcut, ShortcutManager};

fn main() {
    let shortcuts = vec![
        Shortcut {
            sequence: "gs".into(),
            command: "git status".into(),
            execute: true,
            description: Some("Status of the git project".to_string()),
        },
        Shortcut {
            sequence: "v".into(),
            command: "vim ".into(),
            execute: false,
            description: None,
        },
        Shortcut {
            sequence: "ll".into(),
            command: "ls -la".into(),
            execute: true,
            description: Some("List directories".to_string()),
        },
    ];

    let mut manager = ShortcutManager::new(shortcuts);

    if let Ok(shortcut) = manager.run() {
        print!("{}", shortcut)
    }
}
