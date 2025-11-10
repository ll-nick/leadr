#!/usr/bin/env fish

set SCRIPT_DIR (cd (dirname (status -f)) &> /dev/null && pwd)

# Set this directory as the config directory to load the test mappings
set -gx LEADR_CONFIG_DIR "$SCRIPT_DIR"

source "$SCRIPT_DIR/../init.fish"

# Bind Ctrl-G to invoke leadr
bind \cg __leadr_invoke__
