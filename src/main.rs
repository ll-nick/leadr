use leadr::{LeadrError, Shortcut, ShortcutManager, ShortcutResult};

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

    match manager.run() {
        Ok(ShortcutResult::Execute(command)) => print!("#EXEC {}", command),
        Ok(ShortcutResult::Insert(command)) => print!("{}", command),
        Ok(ShortcutResult::NoMatch | ShortcutResult::Cancelled) => {}
        Err(e) => {
            eprintln!(
                "Fatal error: {}",
                match e {
                    LeadrError::TerminalSetup(msg) => msg,
                    LeadrError::ReadError(msg) => msg,
                }
            );
            std::process::exit(1);
        }
    }
}
