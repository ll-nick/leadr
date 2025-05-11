use crate::{Config, LeadrError, keymap::to_bash_binding};

const INIT_SCRIPT_TEMPLATE: &str = include_str!("../shell/init.bash");

pub fn init_bash(config: &Config) -> Result<String, LeadrError> {
    let leader_key = to_bash_binding(&config.leader)?;

    Ok(INIT_SCRIPT_TEMPLATE
        .replace("{{bind_key}}", &leader_key)
        .replace("{{exec_prefix}}", &config.exec_prefix))
}
