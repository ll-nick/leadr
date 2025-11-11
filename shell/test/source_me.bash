#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

# Set this directory as the config directory to load the test mappings
export LEADR_CONFIG_DIR="$SCRIPT_DIR"

source "$SCRIPT_DIR/../init.bash"

# Bind Ctrl-G to invoke leadr
bind -m emacs -x '"\C-g":__leadr_invoke__'
bind -m vi-insert -x '"\C-g":__leadr_invoke__'
# In vi-command mode, switch to insert mode, invoke leadr using the binding defined above, then return to command mode
bind -m vi-command '"\C-g":i__leadr_invoke__\e'
