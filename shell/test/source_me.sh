#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

export LEADR_CONFIG_DIR="$SCRIPT_DIR"

BIND_KEY="\C-g"

CURRENT_SHELL=$(basename "$SHELL")
case "$CURRENT_SHELL" in
    bash)
        INIT_FILE="$SCRIPT_DIR/../init.bash"
        ;;
    zsh)
        INIT_FILE="$SCRIPT_DIR/../init.zsh"
        ;;
    *)
        echo "Unsupported shell: $CURRENT_SHELL"
        return 1 2>/dev/null || exit 1
        ;;
esac

source <(sed "s/{{bind_key}}/$BIND_KEY/" "$INIT_FILE")
