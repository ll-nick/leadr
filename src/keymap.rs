use std::collections::HashMap;

use crate::LeadrError;

pub fn to_ascii(key: &str) -> Result<String, LeadrError> {
    let mut map = HashMap::new();
    map.insert("<C-Space>", "\\x00");
    map.insert("<C-a>", "\\x01");
    map.insert("<C-b>", "\\x02");
    map.insert("<C-d>", "\\x04");
    map.insert("<C-e>", "\\x05");
    map.insert("<C-f>", "\\x06");
    map.insert("<C-g>", "\\x07");
    map.insert("<C-h>", "\\x08");
    map.insert("<C-i>", "\\x09");
    map.insert("<C-j>", "\\x0A");
    map.insert("<C-k>", "\\x0B");
    map.insert("<C-l>", "\\x0C");
    map.insert("<C-m>", "\\x0D");
    map.insert("<C-n>", "\\x0E");
    map.insert("<C-o>", "\\x0F");
    map.insert("<C-p>", "\\x10");
    map.insert("<C-q>", "\\x11");
    map.insert("<C-r>", "\\x12");
    map.insert("<C-s>", "\\x13");
    map.insert("<C-t>", "\\x14");
    map.insert("<C-u>", "\\x15");
    map.insert("<C-v>", "\\x16");
    map.insert("<C-w>", "\\x17");
    map.insert("<C-x>", "\\x18");
    map.insert("<C-y>", "\\x19");
    map.insert("<C-z>", "\\x1A");

    match map.get(key) {
        Some(&binding) => Ok(binding.to_string()),
        None => Err(LeadrError::InvalidKeymapError(key.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LeadrError;

    #[test]
    fn test_valid_key_conversion() {
        assert_eq!(to_ascii("<C-a>").unwrap(), "\\x01");
        assert_eq!(to_ascii("<C-Space>").unwrap(), "\\x00");
    }

    #[test]
    fn test_invalid_key_conversion() {
        let err = to_ascii("<C-unknown>");
        assert!(matches!(err, Err(LeadrError::InvalidKeymapError(_))));
    }
}
