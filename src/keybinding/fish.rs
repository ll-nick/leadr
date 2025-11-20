use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Converts a KeyEvent to a Fish keybinding
pub fn fish_keyevent_to_shell_seq(ev: KeyEvent) -> String {
    use KeyCode::*;
    use KeyModifiers as KM;

    // --- Determine modifier string ---
    let ctrl = ev.modifiers.contains(KM::CONTROL);
    let alt = ev.modifiers.contains(KM::ALT);
    let shift = ev.modifiers.contains(KM::SHIFT);

    let modifier = match (ctrl, alt, shift) {
        (false, false, false) => "",
        (true, false, false) => "ctrl-",
        (false, true, false) => "alt-",
        (false, false, true) => "shift-",
        (true, true, false) => "ctrl-alt-",
        (true, false, true) => "ctrl-shift-",
        (false, true, true) => "alt-shift-",
        (true, true, true) => "ctrl-alt-shift-",
    }
    .to_string();

    // --- Determine keycode string ---
    let keycode = match ev.code {
        Backspace => "backspace".into(),
        Enter => "enter".into(),
        Esc => "escape".into(),
        Tab => "tab".into(),
        Char(' ') => "space".into(),
        Char('-') => "minus".into(),
        Char(',') => "comma".into(),
        Char(c) => c.into(),
        Delete => "delete".into(),
        Insert => "insert".into(),
        Home => "home".into(),
        End => "end".into(),
        PageUp => "pageup".into(),
        PageDown => "pagedown".into(),
        Up => "up".into(),
        Down => "down".into(),
        Left => "left".into(),
        Right => "right".into(),
        F(n) => format!("f{}", n),
        _ => "".into(),
    };

    return format!("{}{}", modifier, keycode);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keybinding::parse::parse_keysequence;

    #[test]
    fn test_fish_fields_simple() {
        let seq = fish_keyevent_to_shell_seq(parse_keysequence("<C-g>").unwrap()[0].clone());
        assert_eq!(seq, "ctrl-g");
    }

    #[test]
    fn test_fish_fields_combo() {
        let seq = fish_keyevent_to_shell_seq(parse_keysequence("<C-M-S-x>").unwrap()[0].clone());
        assert_eq!(seq, "ctrl-alt-shift-x");
    }

    #[test]
    fn test_fish_fields_non_char() {
        let seq = fish_keyevent_to_shell_seq(parse_keysequence("<F5>").unwrap()[0].clone());
        assert_eq!(seq, "f5");
    }


}
