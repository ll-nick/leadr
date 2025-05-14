use crate::{keymap::to_ascii, Config, LeadrError};

const BASH_INIT_TEMPLATE: &str = include_str!("../shell/init.bash");
const ZSH_INIT_TEMPLATE: &str = include_str!("../shell/init.zsh");

pub fn init_bash(config: &Config) -> Result<String, LeadrError> {
    let leader_key = to_ascii(&config.leadr_key)?;

    Ok(BASH_INIT_TEMPLATE
        .replace("{{bind_key}}", &leader_key)
        .replace("{{exec_prefix}}", &config.exec_prefix))
}

pub fn init_zsh(config: &Config) -> Result<String, LeadrError> {
    let leader_key = to_ascii(&config.leadr_key)?;

    Ok(ZSH_INIT_TEMPLATE
        .replace("{{bind_key}}", &leader_key)
        .replace("{{exec_prefix}}", &config.exec_prefix))
}
