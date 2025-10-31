use crate::{
    Config, LeadrError,
    keymap::{Shell, keyevents_to_shell_binding},
};

const BASH_INIT_TEMPLATE: &str = include_str!("../shell/init.bash");
const ZSH_INIT_TEMPLATE: &str = include_str!("../shell/init.zsh");
const INIT_FUNCTION_NAME: &str = "__leadr_invoke__";

/// Generates a bash script that handles the resulting command and binds it to the leadr key.
pub fn init_bash(config: &Config) -> Result<String, LeadrError> {
    let bind_command =
        keyevents_to_shell_binding(&config.leadr_key_events()?, INIT_FUNCTION_NAME, Shell::Bash)?;

    let script = BASH_INIT_TEMPLATE.to_owned();
    Ok(script + bind_command.as_str())
}

/// Generates a zsh script that handles the resulting command and binds it to the leadr key.
pub fn init_zsh(config: &Config) -> Result<String, LeadrError> {
    let bind_command =
        keyevents_to_shell_binding(&config.leadr_key_events()?, INIT_FUNCTION_NAME, Shell::Zsh)?;

    let script = ZSH_INIT_TEMPLATE.to_owned();
    Ok(script + bind_command.as_str())
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
