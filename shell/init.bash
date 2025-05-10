__leadr_invoke__() {
    local output
    # output=$(leadr)
    output="git status"
    if [[ $? -eq 0 ]]; then
        READLINE_LINE="$output"
        READLINE_POINT=${#READLINE_LINE}
    fi
}

bind -x '"\C-@":__leadr_invoke__' # Ctrl-Space (C-@)
