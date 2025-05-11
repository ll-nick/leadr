use clap::Parser;

use leadr::{Config, ShortcutHandler, ShortcutResult};

#[derive(Parser)]
#[command(name = "leadr")]
#[command(about = "Minimal shell shortcut launcher")]
struct Cli {
    #[arg(long)]
    bash: bool,
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

    if cli.bash {
        match leadr::shell::init_bash(&config) {
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

    if cli.list {
        println!("{}", config.render_table());
        return;
    }

    let mut handler = ShortcutHandler::new(config.shortcuts, config.padding);

    match handler.run() {
        Ok(ShortcutResult::Shortcut(shortcut)) => {
            print!("{}", shortcut.format_command(&config.exec_prefix))
        }
        Ok(ShortcutResult::NoMatch | ShortcutResult::Cancelled) => {}
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            std::process::exit(1);
        }
    }
}
