# === Configurable Variables ===
LEADR_BIND_KEY='\C-@' # Ctrl-Space
LEADR_EXEC_PREFIX='#EXEC'
LEADR_CMD_COLOR='\e[1;32m' # Bold green
LEADR_RESET_COLOR='\e[0m'

# === Handle leadr output ===
__leadr_invoke__() {
    local cmd
    cmd="#EXEC git status"

    if [[ "$cmd" =~ ^${LEADR_EXEC_PREFIX}[[:space:]]+(.*) ]]; then
        # If using the exec prefix , run the command right away and manually append to history
        # Unfortunately, there is no easy way to simulate a user pressing enter,
        # so this is the best we can do without additional dependencies
        local actual_cmd="${BASH_REMATCH[1]}"
        printf "${LEADR_CMD_COLOR}%s${LEADR_RESET_COLOR}\n" "$actual_cmd"
        history -s "$actual_cmd"
        eval "$actual_cmd"
    else
        # Otherwise, just prepare the command for the user to run
        READLINE_LINE="$cmd"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

# === Key Binding ===
bind -x "\"${LEADR_BIND_KEY}\":__leadr_invoke__"
