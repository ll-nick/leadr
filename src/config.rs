use std::path::Path;

use crate::{LeadrError, ui::panel::Config as PanelConfig};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// The key binding to activate leadr.
    pub leadr_key: String,

    pub redraw_prompt_line: bool,

    /// Configuration for the keybinding panel.
    pub panel: PanelConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            leadr_key: "<C-g>".into(),
            redraw_prompt_line: true,
            panel: PanelConfig::default(),
        }
    }
}

impl Config {
    pub fn load(config_dir: &Path) -> Result<Self, LeadrError> {
        let config_path = config_dir.join("config.toml");
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            let config = toml::from_str(&contents)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn create_default(config_dir: &Path) -> Result<(), LeadrError> {
        std::fs::create_dir_all(config_dir)?;
        let config_path = config_dir.join("config.toml");
        if !config_path.exists() {
            let default_config = Config::default();
            let contents = toml::to_string(&default_config)?;
            std::fs::write(config_path, contents)?;
        }
        Ok(())
    }
}
