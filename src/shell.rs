use crate::{config::Config, keymap::to_bash_binding};

pub fn init_bash(config: &Config) -> String {
    let leader_key = to_bash_binding(&config.leader).unwrap_or_else(|| <&str>::from("\\C-@")); // Default to Ctrl-Space

    format!(
        r#"# === Configurable Variables ===
LEADR_BIND_KEY='{leader_key}'  # From config
LEADR_EXEC_PREFIX='#EXEC'
LEADR_CMD_COLOR='\e[1;32m' # Bold green
LEADR_RESET_COLOR='\e[0m'

# === Handle leadr output ===
__leadr_invoke__() {{
    local cmd
    cmd="$(leadr)"

    if [[ "$cmd" =~ ^${{LEADR_EXEC_PREFIX}}[[:space:]]+(.*) ]]; then
        local actual_cmd="${{BASH_REMATCH[1]}}"
        printf "${{LEADR_CMD_COLOR}}%s${{LEADR_RESET_COLOR}}\n" "$actual_cmd"
        history -s "$actual_cmd"
        eval "$actual_cmd"
    else
        READLINE_LINE="$cmd"
        READLINE_POINT=${{#READLINE_LINE}}
    fi
}}

# === Key Binding ===
bind -x "\"${{LEADR_BIND_KEY}}\":__leadr_invoke__"
"#,
        leader_key = leader_key
    )
}
