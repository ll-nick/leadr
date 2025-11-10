#!/usr/bin/env nu

# Set this directory as the config directory to load the test mappings
$env.LEADR_CONFIG_DIR = ($env.FILE_PWD)

source ../init.nu

# Add a keybinding for testing
$env.config.keybindings ++= [{
    name: leadr
    modifier: Control
    keycode: Char_g
    mode: [emacs vi_insert vi_normal]
    event: {
        send: executehostcommand
        cmd: "__leadr_invoke__"
    }
}]
