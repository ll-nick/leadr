# === Configurable Variables ===
LEADR_BIND_KEY='{{bind_key}}'
LEADR_EXEC_PREFIX='{{exec_prefix}}'

# === Style Variables ===
LEADR_CMD_COLOR='\e[1;32m' # Bold green
LEADR_RESET_COLOR='\e[0m'

# === Handle leadr output ===
__leadr_invoke__() {
    local cmd
    cmd="$(leadr)"

    if [[ "$cmd" =~ ^${LEADR_EXEC_PREFIX}[[:space:]]+(.*) ]]; then
        # If using the exec prefix , run the command right away.
        local actual_cmd="${BASH_REMATCH[1]}"
        if [[ -n "$TMUX" ]]; then
            # When using tmux, this is simple:
            tmux send-keys "$actual_cmd" Enter
        else
            # Without tmux, there is no easy way to simulate a user pressing enter,
            # so this is the best we can do without additional dependencies.
            printf "${LEADR_CMD_COLOR}%s${LEADR_RESET_COLOR}\n" "$actual_cmd"
            history -s "$actual_cmd"
            eval "$actual_cmd"
        fi
    else
        # Otherwise, just prepare the command for the user to run
        READLINE_LINE="$cmd"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

# === Key Binding ===
bind -x "\"${LEADR_BIND_KEY}\":__leadr_invoke__"
