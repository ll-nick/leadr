use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::error::LeadrError;

/// All currently supported shells
pub enum Shell {
    Bash,
    Zsh,
}

/// Parses a single Vim-style key like `<C-x>`, `<M-Enter>`, `<F5>`.
fn parse_vim_key(key: &str) -> Result<KeyEvent, LeadrError> {
    let key = key.trim_matches(|c| c == '<' || c == '>');
    let parts: Vec<&str> = key.split('-').collect();

    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let base_key;

    if parts.len() > 1 {
        // Treat first N-1 as modifiers, last as base key
        for part in &parts[..parts.len() - 1] {
            match part.to_uppercase().as_str() {
                "C" => ctrl = true,
                "M" => alt = true,
                "S" => shift = true,
                other => {
                    return Err(LeadrError::InvalidKeymapError(format!(
                        "Unknown modifier '{}'",
                        other
                    )));
                }
            }
        }
        base_key = parts[parts.len() - 1].to_string();
    } else {
        // Single element: literal
        base_key = parts[0].to_string();
    }

    let code = match base_key {
        k if k.len() == 1 => KeyCode::Char(k.chars().next().unwrap()),
        k => match k.to_uppercase().as_str() {
            "SPACE" => KeyCode::Char(' '),
            "CR" | "ENTER" => KeyCode::Enter,
            "TAB" => KeyCode::Tab,
            "ESC" => KeyCode::Esc,
            "UP" => KeyCode::Up,
            "DOWN" => KeyCode::Down,
            "LEFT" => KeyCode::Left,
            "RIGHT" => KeyCode::Right,
            k if k.starts_with('F') => {
                let n = k[1..].parse::<u8>().map_err(|_| {
                    LeadrError::InvalidKeymapError(format!("Invalid function key: {}", key))
                })?;
                KeyCode::F(n)
            }
            _ => {
                return Err(LeadrError::InvalidKeymapError(format!(
                    "Unknown key: {}",
                    key
                )));
            }
        },
    };

    let mut modifiers = KeyModifiers::empty();
    if ctrl {
        modifiers |= KeyModifiers::CONTROL;
    }
    if alt {
        modifiers |= KeyModifiers::ALT;
    }
    if shift {
        modifiers |= KeyModifiers::SHIFT;
    }

    Ok(KeyEvent {
        code,
        modifiers,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    })
}

/// Converts a KeyEvent to a Bash/Zsh-compatible sequence
fn keyevent_to_shell_seq(event: KeyEvent) -> String {
    use KeyCode::*;
    let mut s = String::new();

    if event.modifiers.contains(KeyModifiers::ALT) {
        s.push('\x1B'); // ESC prefix for Alt
    }

    match event.code {
        Char(c) => {
            let c = if event.modifiers.contains(KeyModifiers::CONTROL) {
                (c as u8 & 0x1F) as char
            } else if event.modifiers.contains(KeyModifiers::SHIFT) {
                c.to_ascii_uppercase()
            } else {
                c
            };
            s.push(c);
        }
        Enter => s.push('\x0D'),
        Tab => s.push('\x09'),
        Esc => s.push('\x1B'),
        Up => s.push_str("\x1B[A"),
        Down => s.push_str("\x1B[B"),
        Left => s.push_str("\x1B[D"),
        Right => s.push_str("\x1B[C"),
        F(n) => s.push_str(match n {
            1 => "\x1BOP",
            2 => "\x1BOQ",
            3 => "\x1BOR",
            4 => "\x1BOS",
            5 => "\x1B[15~",
            6 => "\x1B[17~",
            7 => "\x1B[18~",
            8 => "\x1B[19~",
            9 => "\x1B[20~",
            10 => "\x1B[21~",
            11 => "\x1B[23~",
            12 => "\x1B[24~",
            _ => "",
        }),
        _ => {}
    }

    s
}

/// Parses a full Vim-style sequence like `<C-x><M-Enter>` into a vector of KeyEvents
pub fn parse_keysequence(seq: &str) -> Result<Vec<KeyEvent>, LeadrError> {
    let mut result = Vec::new();
    let mut current_combo = String::new();
    let mut in_angle = false;

    for char in seq.chars() {
        if char == '<' {
            in_angle = true;
            current_combo.push(char);
        } else if char == '>' && in_angle {
            current_combo.push(char);
            in_angle = false;
            let key_event = parse_vim_key(&current_combo)?;
            result.push(key_event);
            current_combo.clear();
        } else if in_angle {
            current_combo.push(char);
        } else {
            // plain character
            result.push(KeyEvent {
                code: KeyCode::Char(char),
                modifiers: KeyModifiers::empty(),
                kind: crossterm::event::KeyEventKind::Press,
                state: crossterm::event::KeyEventState::NONE,
            });
        }
    }

    Ok(result)
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_parse_simple_char() {
        let ev = parse_vim_key("x").unwrap();
        assert_eq!(ev.code, KeyCode::Char('x'));
        assert!(ev.modifiers.is_empty());
    }

    #[test]
    fn test_parse_ctrl_char() {
        let ev = parse_vim_key("<C-a>").unwrap();
        assert_eq!(ev.code, KeyCode::Char('a'));
        assert!(ev.modifiers.contains(KeyModifiers::CONTROL));
    }

    #[test]
    fn test_parse_alt_shift_char() {
        let ev = parse_vim_key("<M-S-x>").unwrap();
        assert_eq!(ev.code, KeyCode::Char("x".chars().next().unwrap()));
        assert!(ev.modifiers.contains(KeyModifiers::ALT));
        assert!(ev.modifiers.contains(KeyModifiers::SHIFT));
    }

    #[test]
    fn test_parse_named_keys() {
        assert_eq!(parse_vim_key("<Enter>").unwrap().code, KeyCode::Enter);
        assert_eq!(parse_vim_key("<Tab>").unwrap().code, KeyCode::Tab);
        assert_eq!(parse_vim_key("<Esc>").unwrap().code, KeyCode::Esc);
        assert_eq!(parse_vim_key("<Up>").unwrap().code, KeyCode::Up);
        assert_eq!(parse_vim_key("<Down>").unwrap().code, KeyCode::Down);
        assert_eq!(parse_vim_key("<Left>").unwrap().code, KeyCode::Left);
        assert_eq!(parse_vim_key("<Right>").unwrap().code, KeyCode::Right);
    }

    #[test]
    fn test_parse_function_keys() {
        assert_eq!(parse_vim_key("<F1>").unwrap().code, KeyCode::F(1));
        assert_eq!(parse_vim_key("<F12>").unwrap().code, KeyCode::F(12));
        assert!(parse_vim_key("<F13>").is_ok()); // still parsed, but no mapping in keyevent_to_shell_seq
    }

    #[test]
    fn test_keyevent_to_shell_seq_ctrl() {
        let ev = parse_vim_key("<C-g>").unwrap();
        let seq = keyevent_to_shell_seq(ev);
        assert_eq!(seq, "\x07"); // Ctrl-G
    }

    #[test]
    fn test_keyevent_to_shell_seq_alt() {
        let ev = parse_vim_key("<M-x>").unwrap();
        let seq = keyevent_to_shell_seq(ev);
        assert_eq!(seq, "\x1Bx"); // ESC + 'x'
    }

    #[test]
    fn test_invalid_modifier() {
        let err = parse_vim_key("<Q-x>").unwrap_err();
        match err {
            LeadrError::InvalidKeymapError(s) => {
                assert!(s.contains("Unknown modifier"));
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_invalid_key() {
        let err = parse_vim_key("<C-NotAKey>").unwrap_err();
        match err {
            LeadrError::InvalidKeymapError(s) => {
                assert!(s.contains("Unknown key"));
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_parse_keysequence_simple() {
        let events = crate::keymap::parse_keysequence("abc").unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].code, KeyCode::Char('a'));
        assert_eq!(events[1].code, KeyCode::Char('b'));
        assert_eq!(events[2].code, KeyCode::Char('c'));
    }

    #[test]
    fn test_parse_keysequence_vim_style() {
        let events = crate::keymap::parse_keysequence("<C-x><M-Enter>").unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].code, KeyCode::Char('x'));
        assert!(events[0].modifiers.contains(KeyModifiers::CONTROL));
        assert_eq!(events[1].code, KeyCode::Enter);
        assert!(events[1].modifiers.contains(KeyModifiers::ALT));
    }

    #[test]
    fn test_keysequence_to_shell_seq_basic() {
        let events = crate::keymap::parse_keysequence("<C-g>").unwrap();
        let seq = crate::keymap::keyevent_to_shell_seq(*events.first().unwrap());
        assert_eq!(seq, "\x07"); // Ctrl-G
    }

    #[test]
    fn test_keysequence_to_shell_seq_alt() {
        let events = crate::keymap::parse_keysequence("<M-x>").unwrap();
        let seq = crate::keymap::keyevent_to_shell_seq(*events.first().unwrap());
        assert_eq!(seq, "\x1Bx"); // ESC + 'x'
    }
}
