use crossterm::event::KeyEvent;

use crate::{
    error::LeadrError,
    keybinding::{bash_zsh::keyevent_to_shell_seq, nushell::nushell_keyevent_to_fields},
};

/// All currently supported shells
pub enum Shell {
    Bash,
    Nushell,
    Zsh,
}

/// Generate shell code to bind a sequence of KeyEvents to a shell function
pub fn keyevents_to_shell_binding(
    events: &[KeyEvent],
    function_name: &str,
    shell: Shell,
) -> Result<String, LeadrError> {
    if events.is_empty() {
        return Err(LeadrError::InvalidKeymapError(
            "No key events provided".into(),
        ));
    }

    match shell {
        Shell::Bash => {
            let key_code_string = events
                .iter()
                .map(|ev| keyevent_to_shell_seq(*ev))
                .collect::<String>();

            return Ok(format!(
                "\nbind -m emacs -x '\"{}\":{}'\n\
                 bind -m vi-insert -x '\"{}\":{}'\n\
                 # In vi-command mode, switch to insert mode, invoke leadr using the binding defined above, then return to command mode\n\
                 bind -m vi-command '\"{}\":i{}\\e'\n",
                key_code_string,
                function_name,
                key_code_string,
                function_name,
                key_code_string,
                function_name,
            ));
        }
        Shell::Zsh => {
            let key_code_string = events
                .iter()
                .map(|ev| keyevent_to_shell_seq(*ev))
                .collect::<String>();

            return Ok(format!(
                "zle -N {}\n\
                bindkey '{}' {}",
                function_name, key_code_string, function_name
            ));
        }
        Shell::Nushell => {
            if events.len() != 1 {
                return Err(LeadrError::InvalidKeymapError(
                    "Nushell only supports single-chord keybindings (e.g. `<C-g>` or <S-M-a>). \
                     Multi-chord sequences like `<C-x><C-f>` are not supported. \
                     Please adjust the leadr keymap in your configuration accordingly."
                        .into(),
                ));
            }

            let event = events[0];
            let fields = nushell_keyevent_to_fields(event)?;

            Ok(format!(
                "\n$env.config.keybindings ++= [{{\n    \
                     name: leadr\n    \
                     modifier: {}\n    \
                     keycode: {}\n    \
                     mode: [emacs vi_insert vi_normal]\n    \
                     event: {{\n        \
                         send: executehostcommand\n        \
                         cmd: \"{}\"\n    \
                     }}\n\
                 }}]",
                fields.modifier, fields.keycode, function_name
            ))
        }
    }
}
