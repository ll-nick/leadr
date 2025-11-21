use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Converts a KeyEvent to a Bash/Zsh-compatible input sequence
pub fn keyevent_to_shell_seq(event: KeyEvent) -> String {
    use KeyCode::*;
    let mut sequence = String::new();

    if event.modifiers.contains(KeyModifiers::ALT) {
        sequence.push('\x1B'); // ESC prefix for Alt
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
            sequence.push(c);
        }
        Enter => sequence.push('\x0D'),
        Tab => sequence.push('\x09'),
        Esc => sequence.push('\x1B'),
        Up => sequence.push_str("\x1B[A"),
        Down => sequence.push_str("\x1B[B"),
        Left => sequence.push_str("\x1B[D"),
        Right => sequence.push_str("\x1B[C"),
        F(n) => sequence.push_str(match n {
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

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::keybinding::parse::parse_keysequence;

    #[test]
    fn test_keyevent_to_shell_seq_ctrl() {
        let seq = keyevent_to_shell_seq(parse_keysequence("<C-g>").unwrap()[0]);
        assert_eq!(seq, "\x07"); // Ctrl-G
    }

    #[test]
    fn test_keyevent_to_shell_seq_alt() {
        let seq = keyevent_to_shell_seq(parse_keysequence("<M-x>").unwrap()[0]);
        assert_eq!(seq, "\x1Bx"); // ESC + 'x'
    }
}
