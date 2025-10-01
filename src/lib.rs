//! leadr: Shell aliases on steroids
//!
//! Define key sequences that expand into commands.
//! Inspired by the (Neo)Vim leader key.

pub mod config;
pub mod error;
pub mod input;
pub mod keymap;
pub mod mappings;
pub mod session;
pub mod shell;
pub mod ui;

pub use config::Config;
pub use error::LeadrError;
pub use mappings::{InsertType, Mapping, Mappings};
pub use session::{LeadrSession, SessionResult};
pub use shell::init_bash;
pub use shell::init_zsh;
pub use shell::init_fish;
pub use ui::{panel::Panel, theme::Theme};
