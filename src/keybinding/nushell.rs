use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::error::LeadrError;

/// Represents a single Nushell keybinding entry
pub struct NushellKeyFields {
    pub modifier: String,
    pub keycode: String,
}

pub fn nushell_keyevent_to_fields(ev: KeyEvent) -> Result<NushellKeyFields, LeadrError> {
    use KeyCode::*;
    use KeyModifiers as KM;

    // --- Determine modifier string ---
    let ctrl = ev.modifiers.contains(KM::CONTROL);
    let alt = ev.modifiers.contains(KM::ALT);
    let shift = ev.modifiers.contains(KM::SHIFT);

    let modifier = match (ctrl, alt, shift) {
        (false, false, false) => "None",
        (true, false, false) => "Control",
        (false, true, false) => "Alt",
        (false, false, true) => "Shift",
        (true, true, false) => "Control_Alt",
        (true, false, true) => "Control_Shift",
        (false, true, true) => "Alt_Shift",
        (true, true, true) => "Control_Alt_Shift",
    }
    .to_string();

    // --- Determine keycode string ---
    let keycode = match ev.code {
        Backspace => "Backspace".into(),
        Enter => "Enter".into(),
        Esc => "Esc".into(),
        Tab => "Tab".into(),
        Char(' ') => "Space".into(),
        Char(c) => format!("Char_{}", c),
        Delete => "Delete".into(),
        Insert => "Insert".into(),
        Home => "Home".into(),
        End => "End".into(),
        PageUp => "PageUp".into(),
        PageDown => "PageDown".into(),
        Up => "Up".into(),
        Down => "Down".into(),
        Left => "Left".into(),
        Right => "Right".into(),
        F(n) => format!("F{}", n),
        Null => "Null".into(),
        BackTab => "BackTab".into(),
        _ => {
            return Err(LeadrError::InvalidKeymapError(format!(
                "Unsupported keycode for Nushell: {:?}",
                ev.code
            )));
        }
    };

    Ok(NushellKeyFields { modifier, keycode })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keybinding::parse::parse_keysequence;

    #[test]
    fn test_nushell_fields_simple() {
        let ev = parse_keysequence("<C-g>").unwrap();
        let fields = nushell_keyevent_to_fields(ev.first().unwrap().clone()).unwrap();
        assert_eq!(fields.modifier, "Control");
        assert_eq!(fields.keycode, "Char_g");
    }

    #[test]
    fn test_nushell_fields_combo() {
        let ev = parse_keysequence("<C-M-S-x>").unwrap();
        let fields = nushell_keyevent_to_fields(ev.first().unwrap().clone()).unwrap();
        assert_eq!(fields.modifier, "Control_Alt_Shift");
        assert_eq!(fields.keycode, "Char_x");
    }

    #[test]
    fn test_nushell_fields_non_char() {
        let ev = parse_keysequence("<F5>").unwrap();
        let fields = nushell_keyevent_to_fields(ev.first().unwrap().clone()).unwrap();
        assert_eq!(fields.keycode, "F5");
        assert_eq!(fields.modifier, "None");
    }
}

