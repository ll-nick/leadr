//! leadr: Shell aliases on steroids
//!
//! Define key sequences that expand into commands.
//! Inspired by the (Neo)Vim leader key.

use clap::Parser;

use leadr::{Config, ShortcutHandler, ShortcutResult};

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[arg(long)]
    bash: bool,
    #[arg(long, short = 'l', help = "List all shortcuts")]
    list: bool,
    #[arg(long)]
    zsh: bool,
}

fn main() {
    let cli = Cli::parse();

    let config: Config = match confy::load("leadr", "config") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config");
            match &e {
                confy::ConfyError::BadTomlData(inner) => {
                    eprintln!("TOML error: {}", inner);
                }
                _ => eprintln!("Error: {}", e),
            }
            std::process::exit(1);
        }
    };

    if cli.bash {
        match leadr::init_bash(&config) {
            Ok(script) => {
                print!("{}", script);
                return;
            }
            Err(e) => {
                eprintln!("Error generating bash script: {:?}", e);
                std::process::exit(1);
            }
        };
    }
    if cli.zsh {
        match leadr::init_zsh(&config) {
            Ok(script) => {
                print!("{}", script);
                return;
            }
            Err(e) => {
                eprintln!("Error generating zsh script: {:?}", e);
                std::process::exit(1);
            }
        };
    }

    if cli.list {
        println!("{}", config.render_shortcut_table());
        return;
    }

    let mut handler = ShortcutHandler::new(config);

    match handler.run() {
        Ok(ShortcutResult::Shortcut(shortcut)) => {
            print!("{}", shortcut);
        }
        Ok(ShortcutResult::NoMatch | ShortcutResult::Cancelled) => {}
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            std::process::exit(1);
        }
    }
}
