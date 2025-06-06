use serde::{Deserialize, Serialize};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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
            accent: RgbColor {
                r: 137,
                g: 180,
                b: 250,
            },
            background: RgbColor {
                r: 16,
                g: 16,
                b: 26,
            },
            text_highlight_primary: RgbColor {
                r: 250,
                g: 179,
                b: 135,
            },
            text_highlight_secondary: RgbColor {
                r: 245,
                g: 224,
                b: 220,
            },
            text_primary: RgbColor {
                r: 137,
                g: 180,
                b: 250,
            },
            text_secondary: RgbColor {
                r: 108,
                g: 113,
                b: 196,
            },
        }
    }
}

impl std::default::Default for Theme {
    fn default() -> Self {
        Theme::catppuccin_mocha()
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
