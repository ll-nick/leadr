use crate::{Config, LeadrError, keymap::to_ascii};

const BASH_INIT_TEMPLATE: &str = include_str!("../shell/init.bash");
const ZSH_INIT_TEMPLATE: &str = include_str!("../shell/init.zsh");

/// Generates a bash script that handles the resulting command and binds it to the leadr key.
pub fn init_bash(config: &Config) -> Result<String, LeadrError> {
    let leader_key = to_ascii(&config.leadr_key)?;

    Ok(BASH_INIT_TEMPLATE
        .replace("{{bind_key}}", &leader_key)
        .replace(
            "{{cursor_position_encoding}}",
            &config.encoding_strings.cursor_position,
        )
        .replace("{{exec_prefix}}", &config.encoding_strings.exec_prefix)
        .replace(
            "{{insert_prefix}}",
            &config.encoding_strings.insert_prefix,
        )
        .replace(
            "{{prepend_prefix}}",
            &config.encoding_strings.prepend_prefix,
        )
        .replace("{{append_prefix}}", &config.encoding_strings.append_prefix))
}

/// Generates a zsh script that handles the resulting command and binds it to the leadr key.
pub fn init_zsh(config: &Config) -> Result<String, LeadrError> {
    let leader_key = to_ascii(&config.leadr_key)?;

    Ok(ZSH_INIT_TEMPLATE
        .replace("{{bind_key}}", &leader_key)
        .replace(
            "{{cursor_position_encoding}}",
            &config.encoding_strings.cursor_position,
        )
        .replace("{{exec_prefix}}", &config.encoding_strings.exec_prefix)
        .replace(
            "{{prepend_prefix}}",
            &config.encoding_strings.prepend_prefix,
        )
        .replace("{{append_prefix}}", &config.encoding_strings.append_prefix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    #[test]
    fn test_bash_script_contains_replacements() {
        let config = Config::default();
        let result = init_bash(&config).unwrap();
        assert!(result.contains(&config.encoding_strings.cursor_position));
        assert!(result.contains(&config.encoding_strings.exec_prefix));
        assert!(result.contains(&config.encoding_strings.insert_prefix));
        assert!(result.contains(&config.encoding_strings.prepend_prefix));
        assert!(result.contains(&config.encoding_strings.append_prefix));
        assert!(result.contains("\\x07"));
    }

    #[test]
    fn test_zsh_script_contains_replacements() {
        let config = Config::default();
        let result = init_zsh(&config).unwrap();
        assert!(result.contains(&config.encoding_strings.cursor_position));
        assert!(result.contains(&config.encoding_strings.exec_prefix));
        assert!(result.contains(&config.encoding_strings.prepend_prefix));
        assert!(result.contains(&config.encoding_strings.append_prefix));
        assert!(result.contains("\\x07"));
    }
}
