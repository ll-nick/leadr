use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Symbols {
    pub append: String,
    pub arrow: String,
    pub evaluate: String,
    pub execute: String,
    pub insert: String,
    pub prepend: String,
    pub replace: String,
    pub sequence_begin: String,
    pub surround: String,
}

impl std::default::Default for Symbols {
    fn default() -> Self {
        Self {
            append: "󰌒".into(),
            arrow: "→".into(),
            evaluate: "󰊕".into(),
            execute: "󰌑".into(),
            insert: "".into(),
            prepend: "󰌥".into(),
            replace: " ".into(),
            sequence_begin: "󰄾".into(),
            surround: "󰅪".into(),
        }
    }
}
