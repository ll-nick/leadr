# === Configurable Variables ===
LEADR_BIND_KEY='{{bind_key}}'
LEADR_EXEC_PREFIX='{{exec_prefix}}'

# === Style Variables ===
LEADR_CMD_COLOR=$'\e[1;32m'   # Bold green
LEADR_RESET_COLOR=$'\e[0m'

# === Handle leadr output ===
__leadr_invoke__() {
    local cmd
    cmd="$(leadr)"

    if [[ "$cmd" =~ "^${LEADR_EXEC_PREFIX}[[:space:]]+(.*)" ]]; then
        actual_cmd=$match[1]
        if [[ -n "$TMUX" ]]; then
            tmux send-keys "$actual_cmd" Enter
        else
            printf "${LEADR_CMD_COLOR}%s${LEADR_RESET_COLOR}\n" "$actual_cmd"
            print -s -- "$actual_cmd"
            eval "$actual_cmd"
        fi
    else
        LBUFFER+="$cmd"
    fi
    zle reset-prompt
}

# === Key Binding ===
zle -N __leadr_invoke__
bindkey "${LEADR_BIND_KEY}" __leadr_invoke__

