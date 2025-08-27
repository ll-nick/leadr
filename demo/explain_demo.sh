#!/usr/bin/env bash
set -euo pipefail

TYPE_SPEED=20

# Colors
BLUE='\033[1;34m'
RESET='\033[0m'

say() {
    echo -e "$1" | pv -qL $TYPE_SPEED
}

welcome() {
    say "Welcome to leadr! 🎉 Execute commands vim-style: leadr prefix + key sequence 🚀"
    say "👉 Let’s start simple: check the status of this git repo by typing ${BLUE}<Ctrl-G>gs${RESET}."
    say "leadr will insert and execute 'git status' for you."
}

insert() {
    say "That's neat but how’s that better than an alias you ask? 🤔"
    say "So far it isn’t really...so let’s step it up."
    say "We already staged a file. To create a commit type ${BLUE}<Ctrl-G>gc${RESET}."
    say "Notice how the command is only pre-filled - not yet executed - with the cursor right where you need it 🎯."
}

dynamic_content() {
    say "But wait! Before committing, let’s show off that leadr can insert dynamic content:"
    say "${BLUE}<Ctrl-G>id${RESET} inserts today’s date 🗓️."
}

advanced_example() {
    say "Now let's go for a more advanced example by finding and checking out a very specific commit. We'll do that all fancy using fuzzy finding."
    say "${BLUE}<Ctrl-G>gl${RESET} shows 'git log --oneline' 📜."
}

append_to_command() {
    say "To find the commit, we append a pipe to fzf and slice a field with awk."
    say "We can append that all at once with ${BLUE}<Ctrl-G>fl${RESET}."
    say "The column we want is the first one, so we just need to type '1' and hit Enter."
}

surround_command() {
    say "Great, we got it! But we don't really care for the commit hash itself, we just want to check it out."
    say "No worries, we can surround the previous command with a ${BLUE}\$()${RESET} with our cursor conveniently placed in the beginning so we just need to type 'git checkout'."
}

panel() {
    say "Too much to remember? 🤯 leadr has got you covered!"
    say "After a short delay, a neat little panel pops up - NeoVim enjoyers will feel right at home ♥️."
    say "The top section shows available commands given the next key you type with little icons representing the type of mapping."
    say "The currently typed key sequence is shown in the bottom left."
}

thanks() {
    say "That’s all folks, thanks for watching! 🎬"
    say "👉 See you on GitHub: github.com/ll-nick/leadr"
}

# ordered list of demo steps
ALL_STEPS=(
    welcome
    insert
    dynamic_content
    advanced_example
    append_to_command
    surround_command
    panel
    thanks
)

# --- dispatch ---
if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <step-name|all>" >&2
    echo "Available steps: ${ALL_STEPS[*]}" >&2
    exit 1
fi

if [[ "$1" == "all" ]]; then
    clear
    for step in "${ALL_STEPS[@]}"; do
        $step
        echo    # blank line between paragraphs
        sleep 1 # small pause between steps
    done
else
    if declare -F "$1" > /dev/null; then
        clear
        $1
    else
        echo "Error: unknown step '$1'" >&2
        echo "Available steps: ${ALL_STEPS[*]}" >&2
        exit 1
    fi
fi
