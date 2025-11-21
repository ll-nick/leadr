use color_eyre::eyre::{Result, ensure};
use crossterm::event::KeyEvent;

use crate::keybinding::{
    bash_zsh::keyevent_to_shell_seq, fish::fish_keyevent_to_shell_seq,
    nushell::nushell_keyevent_to_fields,
};

/// All currently supported shells
pub enum Shell {
    Bash,
    Fish,
    Nushell,
    Zsh,
}

/// Generate shell code to bind a sequence of KeyEvents to a shell function
pub fn keyevents_to_shell_binding(
    events: &[KeyEvent],
    function_name: &str,
    shell: Shell,
) -> Result<String> {
    ensure!(
        !events.is_empty(),
        "Invalid leadr keymap: No key events provided."
    );

    match shell {
        Shell::Bash => {
            let key_code_string = events
                .iter()
                .map(|ev| keyevent_to_shell_seq(*ev))
                .collect::<String>();

            Ok(format!(
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
            ))
        }
        Shell::Fish => {
            let key_code_string = events
                .iter()
                .map(|ev| fish_keyevent_to_shell_seq(*ev) + ",")
                .collect::<String>();

            let key_code_string = key_code_string.trim_end_matches(",");

            Ok(format!("\nbind {} {}\n", key_code_string, function_name))
        }
        Shell::Nushell => {
            ensure!(
                events.len() == 1,
                "Invalid keymap: Nushell only supports single-chord keybindings (e.g. `<C-g>` or <S-M-a>).\n  \
                     Multi-chord sequences like `<C-x><C-f>` are not supported.\n  \
                     Please adjust the leadr keymap in your configuration accordingly."
            );

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
        Shell::Zsh => {
            let key_code_string = events
                .iter()
                .map(|ev| keyevent_to_shell_seq(*ev))
                .collect::<String>();

            Ok(format!(
                "zle -N {}\n\
                bindkey '{}' {}",
                function_name, key_code_string, function_name
            ))
        }
    }
}
