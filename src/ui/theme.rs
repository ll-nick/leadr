use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::LeadrError;

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
    pub fn catppuccin_latte() -> Self {
        Self {
            accent: rgb(30, 102, 245),
            background: rgb(220, 224, 232),
            text_highlight_primary: rgb(254, 100, 11),
            text_highlight_secondary: rgb(220, 138, 120),
            text_primary: rgb(30, 102, 245),
            text_secondary: rgb(156, 160, 176),
        }
    }

    pub fn catppuccin_frappe() -> Self {
        Self {
            accent: rgb(140, 170, 238),
            background: rgb(35, 38, 52),
            text_highlight_primary: rgb(239, 159, 118),
            text_highlight_secondary: rgb(242, 213, 207),
            text_primary: rgb(140, 170, 238),
            text_secondary: rgb(115, 121, 148),
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            accent: rgb(138, 173, 244),
            background: rgb(24, 25, 38),
            text_highlight_primary: rgb(245, 169, 127),
            text_highlight_secondary: rgb(244, 219, 214),
            text_primary: rgb(138, 173, 244),
            text_secondary: rgb(110, 115, 141),
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            accent: rgb(137, 180, 250), // Blue
            background: rgb(17, 17, 27), // Crust
            text_highlight_primary: rgb(250, 179, 135), // Peach
            text_highlight_secondary: rgb(245, 224, 220), // Rosewater
            text_primary: rgb(137, 180, 250), // Blue
            text_secondary: rgb(108, 112, 134), // Overlay0
        }
    }

    pub fn load(config_dir: &Path, theme_name: &str) -> Result<Self, LeadrError> {
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
