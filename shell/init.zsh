# === Configurable Variables ===
LEADR_BIND_KEY='{{bind_key}}'
LEADR_CURSOR_POSITION_ENCODING='{{cursor_position_encoding}}'
LEADR_EXEC_PREFIX='{{exec_prefix}}'
LEADR_INSERT_PREFIX='{{insert_prefix}}'
LEADR_PREPEND_PREFIX='{{prepend_prefix}}'
LEADR_APPEND_PREFIX='{{append_prefix}}'


# === Style Variables ===
LEADR_CMD_COLOR=$'\e[1;32m'   # Bold green
LEADR_RESET_COLOR=$'\e[0m'

# === Handle leadr output ===
__leadr_invoke__() {
    local cmd
    cmd="$(leadr)"

    if [[ "$cmd" =~ "^${LEADR_EXEC_PREFIX} (.*)" ]]; then
        # If using the exec prefix, run the command right away.
        actual_cmd=$match[1]

        # Strip cursor placeholder if present
        actual_cmd="${actual_cmd//$LEADR_CURSOR_POSITION_ENCODING/}"

        if [[ -n "$TMUX" ]]; then
            # When using tmux, this is simple:
            tmux send-keys "$actual_cmd" Enter
        else
            # Without tmux, there is no easy way to simulate a user pressing enter,
            # so this is the best we can do without additional dependencies.
            printf "${LEADR_CMD_COLOR}%s${LEADR_RESET_COLOR}\n" "$actual_cmd"
            print -s -- "$actual_cmd"
            eval "$actual_cmd"
        fi
        zle reset-prompt
        return
    fi

    if [[ "$cmd" =~ "^${LEADR_INSERT_PREFIX} (.*)" ]]; then
        local to_insert=$match[1]
        LBUFFER="${LBUFFER}${to_insert}"
        zle reset-prompt
        return
    fi

    if [[ "$cmd" =~ "^${LEADR_PREPEND_PREFIX} (.*)" ]]; then
        local to_prepend=$match[1]
        local original_cursor=$CURSOR
        BUFFER="${to_prepend}${BUFFER}"
        CURSOR=$((original_cursor + ${#to_prepend}))
        zle reset-prompt
        return
    fi

    if [[ "$cmd" =~ "^${LEADR_APPEND_PREFIX} (.*)" ]]; then
        local to_append=$match[1]
        BUFFER="${BUFFER}${to_append}"
        CURSOR=${#BUFFER}
        # Cursor remains unchanged
        zle reset-prompt
        return
    fi

    if [[ "$cmd" == *"$LEADR_CURSOR_POSITION_ENCODING"* ]]; then
        # Get prefix before placeholder
        local before_cursor="${cmd%%${LEADR_CURSOR_POSITION_ENCODING}*}"
        local after_cursor="${cmd#*${LEADR_CURSOR_POSITION_ENCODING}}"
        # Set command without the placeholder
        BUFFER="${before_cursor}${after_cursor}"
        CURSOR=${#before_cursor}
    else
        # Default: insert entire command at cursor
        LBUFFER+="$cmd"
    fi

    zle reset-prompt
}

# === Key Binding ===
zle -N __leadr_invoke__
bindkey "${LEADR_BIND_KEY}" __leadr_invoke__

