#!/usr/bin/env nu

# Set this directory as the config directory to load the test mappings
$env.LEADR_CONFIG_DIR = ($env.FILE_PWD)

# Source init.nu
# TODO: Replace keybinding with Ctrl-G
source ../init.nu
