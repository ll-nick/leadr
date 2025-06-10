//! leadr: Shell aliases on steroids
//!
//! Define key sequences that expand into commands.
//! Inspired by the (Neo)Vim leader key.

pub mod config;
pub mod error;
pub mod session;
pub mod input;
pub mod keymap;
pub mod shell;
pub mod types;
pub mod ui;

pub use config::Config;
pub use error::LeadrError;
pub use session::LeadrSession;
pub use shell::init_bash;
pub use shell::init_zsh;
pub use types::{Mapping, LeadrResult, Mappings};
pub use ui::theme::Theme;
