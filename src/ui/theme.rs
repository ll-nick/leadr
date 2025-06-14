use std::{fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Theme {
    pub accent: RgbColor,
    pub background: RgbColor,
    pub text_highlight_primary: RgbColor,
    pub text_highlight_secondary: RgbColor,
    pub text_primary: RgbColor,
    pub text_secondary: RgbColor,
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            accent: rgb(137, 180, 250),
            background: rgb(16, 16, 26),
            text_highlight_primary: rgb(250, 179, 135),
            text_highlight_secondary: rgb(245, 224, 220),
            text_primary: rgb(137, 180, 250),
            text_secondary: rgb(108, 113, 196),
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            accent: rgb(138, 173, 244),
            background: rgb(30, 32, 48),
            text_highlight_primary: rgb(238, 190, 190),
            text_highlight_secondary: rgb(202, 211, 245),
            text_primary: rgb(138, 173, 244),
            text_secondary: rgb(166, 173, 200),
        }
    }

    pub fn catppuccin_frappe() -> Self {
        Self {
            accent: rgb(140, 170, 238),
            background: rgb(48, 52, 70),
            text_highlight_primary: rgb(235, 160, 172),
            text_highlight_secondary: rgb(198, 208, 245),
            text_primary: rgb(140, 170, 238),
            text_secondary: rgb(166, 173, 200),
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            accent: rgb(30, 102, 245),
            background: rgb(255, 255, 255),
            text_highlight_primary: rgb(220, 138, 120),
            text_highlight_secondary: rgb(186, 194, 222),
            text_primary: rgb(30, 102, 245),
            text_secondary: rgb(108, 111, 133),
        }
    }

    pub fn load(config_dir: &Path, theme_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let theme = match theme_name {
            "catppuccin-mocha" => Self::catppuccin_mocha(),
            "catppuccin-macchiato" => Self::catppuccin_macchiato(),
            "catppuccin-frappe" => Self::catppuccin_frappe(),
            "catppuccin-latte" => Self::catppuccin_latte(),
            other => {
                let theme_path = config_dir.join("themes").join(format!("{other}.toml"));
                let contents = fs::read_to_string(&theme_path)?;
                toml::from_str(&contents)?
            }
        };
        Ok(theme)
    }
}

impl std::default::Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<RgbColor> for crossterm::style::Color {
    fn from(rgb: RgbColor) -> Self {
        crossterm::style::Color::Rgb {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

fn rgb(r: u8, g: u8, b: u8) -> RgbColor {
    RgbColor { r, g, b }
}
