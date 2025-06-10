//! leadr: Shell aliases on steroids
//!
//! Define key sequences that expand into commands.
//! Inspired by the (Neo)Vim leader key.
use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;

use leadr::{Config, LeadrError, SessionResult, LeadrSession, Theme};

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[arg(long)]
    bash: bool,
    #[arg(long, short = 'l', help = "List all mappings")]
    list: bool,
    #[arg(long)]
    zsh: bool,
}

fn main() {
    let cli = Cli::parse();

    let config_dir = match get_config_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error determining config directory: {}", e);
            std::process::exit(1);
        }
    };
    let config = match Config::load(&config_dir) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };
    if let Err(e) = config.validate() {
        eprintln!("Error validating config: {}", e);
        std::process::exit(1);
    }
    let theme = Theme::default();

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
        println!("{}", config.render_mapping_table());
        return;
    }

    let mut handler = LeadrSession::new(config, theme);

    match handler.run() {
        Ok(SessionResult::Command(command)) => {
            print!("{}", command);
        }
        Ok(SessionResult::NoMatch | SessionResult::Cancelled) => {}
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_config_dir() -> Result<PathBuf, LeadrError> {
    if let Ok(custom_path) = std::env::var("LEADR_CONFIG_DIR") {
        Ok(PathBuf::from(custom_path))
    } else {
        if let Some(path) = ProjectDirs::from("com", "leadr", "leadr") {
            Ok(path.config_dir().to_path_buf())
        } else {
            Err(LeadrError::ConfigDirNotFound)
        }
    }
}
