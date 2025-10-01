use crate::{Config, LeadrError, keymap::parse_keybinding};

const BASH_INIT_TEMPLATE: &str = include_str!("../shell/init.bash");
const ZSH_INIT_TEMPLATE: &str = include_str!("../shell/init.zsh");
const FISH_INIT_TEMPLATE: &str = include_str!("../shell/init.fish");

/// Generates a bash script that handles the resulting command and binds it to the leadr key.
pub fn init_bash(config: &Config) -> Result<String, LeadrError> {
    let leader_key = parse_keybinding(&config.leadr_key)?;

    Ok(BASH_INIT_TEMPLATE.replace("{{bind_key}}", &leader_key))
}

/// Generates a zsh script that handles the resulting command and binds it to the leadr key.
pub fn init_zsh(config: &Config) -> Result<String, LeadrError> {
    let leader_key = parse_keybinding(&config.leadr_key)?;

    Ok(ZSH_INIT_TEMPLATE.replace("{{bind_key}}", &leader_key))
}

/// Generates a fish script that handles the resulting command and binds it to the leadr key.
pub fn init_fish(config: &Config) -> Result<String, LeadrError> {
    let leader_key = parse_keybinding(&config.leadr_key)?;

    Ok(FISH_INIT_TEMPLATE.replace("{{bind_key}}", &leader_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    #[test]
    fn test_bash_script_contains_replacements() {
        let config = Config::default();
        let result = init_bash(&config).unwrap();
        assert!(result.contains("\x07"));
    }

    #[test]
    fn test_zsh_script_contains_replacements() {
        let config = Config::default();
        let result = init_zsh(&config).unwrap();
        assert!(result.contains("\x07"));
    }
}
