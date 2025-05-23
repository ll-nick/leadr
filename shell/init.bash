# === Configurable Variables ===
LEADR_BIND_KEY='{{bind_key}}'
LEADR_CURSOR_POSITION_ENCODING='{{cursor_position_encoding}}'
LEADR_EXEC_PREFIX='{{exec_prefix}}'
LEADR_PREPEND_PREFIX='{{prepend_prefix}}'
LEADR_APPEND_PREFIX='{{append_prefix}}'

# === Style Variables ===
LEADR_CMD_COLOR='\e[1;32m' # Bold green
LEADR_RESET_COLOR='\e[0m'

# === Handle leadr output ===
__leadr_invoke__() {
    local cmd
    cmd="$(leadr)"

    if [[ "$cmd" =~ ^${LEADR_EXEC_PREFIX}\ (.*) ]]; then
        # If using the exec prefix, run the command right away.
        local actual_cmd="${BASH_REMATCH[1]}"

        # Strip cursor placeholder if present
        actual_cmd="${actual_cmd//$LEADR_CURSOR_POSITION_ENCODING/}"

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
        return
    fi

    if [[ "$cmd" =~ ^${LEADR_PREPEND_PREFIX}\ (.*) ]]; then
        local to_prepend="${BASH_REMATCH[1]}"
        local original_point=$READLINE_POINT
        READLINE_LINE="${to_prepend}${READLINE_LINE}"
        READLINE_POINT=$((original_point + ${#to_prepend}))
        return
    fi

    if [[ "$cmd" =~ ^${LEADR_APPEND_PREFIX}\ (.*) ]]; then
        local to_append="${BASH_REMATCH[1]}"
        READLINE_LINE="${READLINE_LINE}${to_append}"
        READLINE_POINT=${#READLINE_LINE}
        return
    fi

    if [[ "$cmd" == *"$LEADR_CURSOR_POSITION_ENCODING"* ]]; then
        # Determine cursor position and prepare line for user
        cursor_pos="${cmd%%$LEADR_CURSOR_POSITION_ENCODING*}"
        # Everything before the cursor placeholder
        # Length of the string before the cursor placeholder
        cursor_pos=${#cursor_pos}
        # Remove the cursor placeholder
        cmd="${cmd//$LEADR_CURSOR_POSITION_ENCODING/}"

        READLINE_LINE="$cmd"
        READLINE_POINT=$cursor_pos
    else
        READLINE_LINE="$cmd"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

# === Key Binding ===
bind -x "\"${LEADR_BIND_KEY}\":__leadr_invoke__"
