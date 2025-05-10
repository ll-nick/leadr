__leadr_invoke__() {
    local output
    cmd="#EXEC git status"

    if [[ "$cmd" =~ ^#EXEC[[:space:]]+(.*) ]]; then
        # If prefixed with #EXEC, run the command right away and manually append to history
        # Unfortunately, there is no easy way to simulate a user pressing enter, so this is the best we can do
        local actual_cmd="${BASH_REMATCH[1]}"
        printf "\e[1;32m%s\e[0m\n" "$actual_cmd"
        history -s "$actual_cmd" # Save to history
        eval "$actual_cmd"       # Run in current shell
    else
        # Otherwise, just prepare the command for the user to run
        READLINE_LINE="$cmd"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

bind -x '"\C-@":__leadr_invoke__' # Ctrl-Space (C-@)
