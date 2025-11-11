#!/usr/bin/env zsh

SCRIPT_DIR=$(cd -- "$(dirname -- "${(%):-%N}")" &> /dev/null && pwd)

# Set this directory as the config directory to loa the test mappings
export LEADR_CONFIG_DIR="$SCRIPT_DIR"

source "$SCRIPT_DIR/../init.zsh"

# Bind Ctrl-G to invoke leadr
zle -N __leadr_invoke__
bindkey '\C-g' __leadr_invoke__

