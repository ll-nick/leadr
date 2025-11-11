__leadr_invoke__() {
    LEADR_COMMAND_POSITION_ENCODING="#COMMAND"
    LEADR_CURSOR_POSITION_ENCODING="#CURSOR"

    leadr_parse_flags() {
        local flag_str="$1"
        local insert=""
        local eval="false"
        local exec="false"

        IFS='+' read -r -a flags_array <<< "$flag_str"
        for flag in "${flags_array[@]}"; do
            case "$flag" in
                REPLACE | INSERT | PREPEND | APPEND | SURROUND) insert="$flag" ;;
                EVAL) eval="true" ;;
                EXEC) exec="true" ;;
            esac
        done

        echo "$insert|$eval|$exec"
    }

    leadr_extract_cursor_pos() {
        local input="$1"
        if [[ "$input" == *"$LEADR_CURSOR_POSITION_ENCODING"* ]]; then
            local before="${input%%$LEADR_CURSOR_POSITION_ENCODING*}"
            echo "${#before}"
        else
            echo "-1"
        fi
    }

    leadr_insert_command() {
        local insert_type="$1"
        local to_insert="$2"
        local cursor_pos="$3"

        local original_point=$READLINE_POINT
        local original_line="$READLINE_LINE"

        case "$insert_type" in
            INSERT)
                READLINE_LINE="${original_line:0:original_point}${to_insert}${original_line:original_point}"
                if [[ $cursor_pos -ge 0 ]]; then
                    READLINE_POINT=$((original_point + cursor_pos))
                else
                    READLINE_POINT=$((original_point + ${#to_insert}))
                fi
                ;;
            PREPEND)
                READLINE_LINE="${to_insert}${original_line}"
                if [[ $cursor_pos -ge 0 ]]; then
                    READLINE_POINT=$cursor_pos
                else
                    READLINE_POINT=$((original_point + ${#to_insert}))
                fi
                ;;
            APPEND)
                READLINE_LINE="${original_line}${to_insert}"
                if [[ $cursor_pos -ge 0 ]]; then
                    READLINE_POINT=$((${#original_line} + cursor_pos))
                else
                    READLINE_POINT=${#READLINE_LINE}
                fi
                ;;
            SURROUND)
                local before_command="${to_insert%%#COMMAND*}"
                local after_command="${to_insert#*#COMMAND}"
                READLINE_LINE="${before_command}${original_line}${after_command}"

                if [[ $cursor_pos -ge 0 ]]; then
                    if [[ $cursor_pos -le ${#before_command} ]]; then
                        # If the cursor position is before the command, we can use it directly
                        READLINE_POINT=$cursor_pos
                    else
                        # If the cursor position is after the command, we need to account for the command expansion
                        READLINE_POINT=$((cursor_pos - ${#LEADR_COMMAND_POSITION_ENCODING} + ${#original_line}))
                    fi
                else
                    READLINE_POINT=$((${#before_command} + original_point))
                fi
                ;;
            *)
                READLINE_LINE="$to_insert"
                if [[ $cursor_pos -ge 0 ]]; then
                    READLINE_POINT=$cursor_pos
                else
                    READLINE_POINT=${#READLINE_LINE}
                fi
                ;;
        esac
    }

    leadr_execute_command() {
        local cmd="$1"
        READLINE_LINE=""
        READLINE_POINT=0
        if [[ -n "$TMUX" ]]; then
            tmux send-keys "$cmd" Enter
        else
            printf "%s\n" "$cmd"
            history -s "$cmd"
            eval "$cmd"
        fi
    }

    leadr_main() {
        local last_prompt_line=$(printf "%s" "${PS1@P}" | tail -n1)
        local current_input="${READLINE_LINE}"

        local cmd="$(
            LEADR_PROMPT="$last_prompt_line" \
                LEADR_CURRENT_INPUT="$current_input" \
                leadr
        )"

        local output_flags="${cmd%% *}"
        local to_insert="${cmd#* }"

        if [[ -z "$cmd" ]]; then
            return
        fi

        local insert_type eval_flag exec_flag
        IFS='|' read -r insert_type eval_flag exec_flag <<< "$(leadr_parse_flags "$output_flags")"

        local cursor_pos="$(leadr_extract_cursor_pos "$to_insert")"
        to_insert="${to_insert//$LEADR_CURSOR_POSITION_ENCODING/}"

        if [[ "$eval_flag" == "true" ]]; then
            to_insert="$(eval "$to_insert")"
            cursor_pos=-1
        fi

        leadr_insert_command "$insert_type" "$to_insert" "$cursor_pos"

        if [[ "$exec_flag" == "true" ]]; then
            leadr_execute_command "$READLINE_LINE"
        fi
    }

    leadr_main
}
