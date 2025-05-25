# === Configurable Variables ===
LEADR_BIND_KEY='\x07'
LEADR_CURSOR_POSITION_ENCODING="#CURSOR"

# === Style Variables ===
LEADR_CMD_COLOR='\e[1;32m' # Bold green
LEADR_RESET_COLOR='\e[0m'

# === Handle leadr output ===
__leadr_invoke__() {
    local cmd output_flags to_insert insert_type eval_flag exec_flag
    local original_point cursor_pos

    cmd="$(leadr)"

    # Split flags and command part
    output_flags="${cmd%% *}"
    to_insert="${cmd#* }"

    # Reset flags
    eval_flag=false
    exec_flag=false

    # Parse flags
    IFS='+' read -r -a flags_array <<< "$output_flags"
    for flag in "${flags_array[@]}"; do
        case "$flag" in
            REPLACE | INSERT | PREPEND | APPEND)
                insert_type="$flag"
                ;;
            EVAL)
                eval_flag=true
                ;;
            EXEC)
                exec_flag=true
                ;;
        esac
    done

    # Handle cursor placeholder in to_insert
    if [[ "$to_insert" == *"$LEADR_CURSOR_POSITION_ENCODING"* ]]; then
        cursor_pos="${to_insert%%$LEADR_CURSOR_POSITION_ENCODING*}"
        cursor_pos=${#cursor_pos}
        to_insert="${to_insert//$LEADR_CURSOR_POSITION_ENCODING/}"
    else
        cursor_pos=-1
    fi

    # If eval is set, evaluate the leadr output before insertion
    if $eval_flag; then
        # Evaluate the to_insert string as a command and capture its output
        # This replaces to_insert with the evaluated string
        to_insert="$(eval "$to_insert")"
        cursor_pos=-1 # Reset cursor position since we evaluated the command
    fi

    # Insert the processed string into the current command line depending on insert_type
    original_point=$READLINE_POINT
    original_line="$READLINE_LINE"
    case "$insert_type" in
        INSERT)
            READLINE_LINE="${READLINE_LINE:0:original_point}${to_insert}${READLINE_LINE:original_point}"
            if [[ -n "$cursor_pos" && $cursor_pos -ge 0 ]]; then
                READLINE_POINT=$((original_point + cursor_pos))
            else
                # If cursor position is not specified, set it to the end of the inserted text
                READLINE_POINT=$((original_point + ${#to_insert}))
            fi
            ;;
        PREPEND)
            READLINE_LINE="${to_insert}${READLINE_LINE}"
            if [[ -n "$cursor_pos" && $cursor_pos -ge 0 ]]; then
                READLINE_POINT=$cursor_pos
            else
                # If cursor position is not specified, set it to where it was before insertion
                READLINE_POINT=$((original_point + ${#to_insert}))
            fi
            ;;
        APPEND)
            READLINE_LINE="${READLINE_LINE}${to_insert}"
            if [[ -n "$cursor_pos" && $cursor_pos -ge 0 ]]; then
                READLINE_POINT=$((${#original_line} + cursor_pos))
            else
                # If cursor position is not specified, set it to the end of the line
                READLINE_POINT=${#READLINE_LINE}
            fi
            ;;
        *)
            # REPLACE case or any other unrecognized type
            READLINE_LINE="$to_insert"
            if [[ -n "$cursor_pos" && $cursor_pos -ge 0 ]]; then
                READLINE_POINT=$cursor_pos
            else
                # If cursor position is not specified, set it to the end of the line
                READLINE_POINT=${#READLINE_LINE}
            fi
            ;;
    esac

    # If exec is set, run the new command line immediately
    if $exec_flag; then
        to_execute="$READLINE_LINE"
        # Reset the line and point to avoid issues with readline
        READLINE_LINE=""
        READLINE_POINT=0
        if [[ -n "$TMUX" ]]; then
            tmux send-keys "$to_execute" Enter
        else
            printf "${LEADR_CMD_COLOR}%s${LEADR_RESET_COLOR}\n" "$to_execute"
            history -s "$to_execute"
            eval "$to_execute"
        fi
    fi
}

# === Key Binding ===
bind -x "\"${LEADR_BIND_KEY}\":__leadr_invoke__"
