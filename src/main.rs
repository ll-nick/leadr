use clap::Parser;
use leadr::{Config, LeadrError, ShortcutHandler, ShortcutResult};

#[derive(Parser)]
#[command(name = "leadr")]
#[command(about = "Minimal shell shortcut launcher")]
struct Cli {
    #[arg(long, short = 'l', help = "List all shortcuts")]
    list: bool,
}

fn main() {
    let cli = Cli::parse();

    let config: Config = match confy::load("leadr", "config") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    if cli.list {
        println!("{}", config.render_table());
        return;
    }

    let mut handler = ShortcutHandler::new(config.shortcuts);

    match handler.run() {
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
