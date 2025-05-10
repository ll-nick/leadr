use std::collections::HashMap;

pub fn to_bash_binding(key: &str) -> Option<&str> {
    let mut map = HashMap::new();
    map.insert("<C-Space>", r#"\C-@"#);

    map.get(key).cloned()
}
