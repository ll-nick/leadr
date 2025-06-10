use std::{path::Path, time::Duration};

use crate::{ui::overlay::Config as OverlayConfig, LeadrError};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// The key binding to activate leadr.
    pub leadr_key: String,

    /// Whether or not to print the ui overlay.
    pub show_overlay: bool,

    /// The duration until the overlay appears.
    pub overlay_timeout: Duration,

    /// The overlay styling.
    pub overlay_style: OverlayConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            leadr_key: "<C-g>".into(),
            show_overlay: true,
            overlay_timeout: Duration::from_millis(500),
            overlay_style: OverlayConfig::default(),
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
}
