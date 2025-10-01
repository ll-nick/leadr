#!/usr/bin/env fish

set SCRIPT_DIR (cd (dirname (status -f)) &> /dev/null && pwd)

# Set this directory as the config directory to load the test mappings
set -gx LEADR_CONFIG_DIR "$SCRIPT_DIR"

# Source init.fish replacing {{bind_key}} with Ctrl-G
source (sed 's/{{bind_key}}/\\cg/' "$SCRIPT_DIR/../init.fish" | psub)