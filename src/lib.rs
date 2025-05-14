//! leadr: Shell aliases on steroids
//!
//! Define key sequences that expand into commands.
//! Inspired by the (Neo)Vim leader key.

pub mod config;
pub mod error;
pub mod handler;
pub mod input;
pub mod keymap;
pub mod shell;
pub mod types;

pub use config::Config;
pub use error::LeadrError;
pub use handler::ShortcutHandler;
pub use shell::init_bash;
pub use shell::init_zsh;
pub use types::{Shortcut, ShortcutResult};
