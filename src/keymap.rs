use crate::LeadrError;
use std::collections::HashMap;

pub fn to_bash_binding(key: &str) -> Result<String, LeadrError> {
    let mut map = HashMap::new();
    map.insert("<C-Space>", r#"\C-@"#);

    match map.get(key) {
        Some(&binding) => Ok(binding.to_string()),
        None => Err(LeadrError::InvalidKeymapError(key.to_string())),
    }
}
