#!/usr/bin/env zsh

SCRIPT_DIR=$(cd -- "$(dirname -- "${(%):-%N}")" &> /dev/null && pwd)

# Set this directory as the config directory to load the test mappings
export LEADR_CONFIG_DIR="$SCRIPT_DIR"

# Source init.zsh replacing {{bind_key}} with Ctrl-G
source <(sed 's/{{bind_key}}/\\C-g/' "$SCRIPT_DIR/../init.zsh")

