use leadr::{Config, LeadrError, ShortcutManager, ShortcutResult};

fn main() {
    let config: Config = match confy::load("leadr", "config") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    let mut manager = ShortcutManager::new(config.shortcuts);

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
