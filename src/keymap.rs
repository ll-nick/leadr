use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::error::LeadrError;

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

/// Parses a full Vim-style sequence like `<C-x><M-Enter>` into a shell string
pub fn parse_keybinding(seq: &str) -> Result<String, LeadrError> {
    let mut result = String::new();
    let mut temp = String::new();
    let mut in_angle = false;

    for c in seq.chars() {
        if c == '<' {
            in_angle = true;
            temp.push(c);
        } else if c == '>' && in_angle {
            temp.push(c);
            in_angle = false;
            let key_event = parse_vim_key(&temp)?;
            result.push_str(&keyevent_to_shell_seq(key_event));
            temp.clear();
        } else if in_angle {
            temp.push(c);
        } else {
            // plain character
            result.push(c);
        }
    }

    Ok(result)
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
    fn test_parse_sequence_mixed() {
        let seq = parse_keybinding("<C-x><M-Enter>abc").unwrap();
        assert_eq!(seq, "\x18\x1B\rabc");
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
}
