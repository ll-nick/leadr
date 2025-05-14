use crate::{Config, LeadrError, keymap::to_ascii};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    #[test]
    fn test_bash_script_contains_replacements() {
        let config = Config::default();
        let result = init_bash(&config).unwrap();
        assert!(result.contains(&config.exec_prefix));
        assert!(result.contains("\\x00"));
    }

    #[test]
    fn test_zsh_script_contains_replacements() {
        let config = Config::default();
        let result = init_zsh(&config).unwrap();
        assert!(result.contains(&config.exec_prefix));
        assert!(result.contains("\\x00"));
    }
}
