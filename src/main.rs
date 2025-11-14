//! leadr: Shell aliases on steroids
//!
//! Define key sequences that expand into commands.
//! Inspired by the (Neo)Vim leader key.

use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::{Result, WrapErr, eyre};
use directories::ProjectDirs;

use leadr::{Config, LeadrSession, Mappings, SessionResult, Theme};

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[arg(long)]
    bash: bool,

    #[arg(long = "init", help = "Create default config files")]
    init: bool,

    #[arg(long, short = 'l', help = "List all mappings")]
    list: bool,

    #[arg(long)]
    nu: bool,

    #[arg(long)]
    zsh: bool,

    /// Generate fish shell initialization script
    #[arg(long)]
    fish: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let config_dir =
        get_config_dir().wrap_err("Failed to determine the configuration directory.")?;

    if cli.init {
        Config::create_default(&config_dir)?;
        Mappings::create_default(&config_dir)?;

        println!("Default config and mappings created in {:?}", config_dir);
        return Ok(());
    }

    let config = Config::load(&config_dir).wrap_err("Failed to load config.")?;
    let mappings = Mappings::load(&config_dir).wrap_err("Failed to load mappings.")?;
    let theme =
        Theme::load(&config_dir, &config.panel.theme_name).wrap_err("Failed to load theme.")?;

    if cli.bash {
        let script =
            leadr::init_bash(&config).wrap_err("Failed to generate Bash initialization script.")?;
        print!("{}", script);
        return Ok(());
    }
    if cli.fish {
        let script =
            leadr::init_fish(&config).wrap_err("Failed to generate Fish initialization script.")?;
        print!("{}", script);
        return Ok(());
    }
    if cli.nu {
        let script = leadr::init_nushell(&config)
            .wrap_err("Failed to generate NuShell initialization script.")?;
        print!("{}", script);
        return Ok(());
    }
    if cli.zsh {
        let script =
            leadr::init_zsh(&config).wrap_err("Failed to generate Zsh initialization script.")?;
        print!("{}", script);
        return Ok(());
    }

    if cli.list {
        println!("{}", mappings.render_table());
        return Ok(());
    }

    let mut session = LeadrSession::new(mappings, config, theme);

    match session.run().wrap_err("Failed to execute leadr session.")? {
        SessionResult::Command(command) => {
            print!("{}", command);
        }
        SessionResult::NoMatch | SessionResult::Cancelled => {}
    }

    Ok(())
}

fn get_config_dir() -> Result<PathBuf> {
    if let Ok(custom_path) = std::env::var("LEADR_CONFIG_DIR") {
        Ok(PathBuf::from(custom_path))
    } else {
        if let Some(path) = ProjectDirs::from("com", "leadr", "leadr") {
            Ok(path.config_dir().to_path_buf())
        } else {
            Err(eyre!("Could not determine configuration directory."))
        }
    }
}
