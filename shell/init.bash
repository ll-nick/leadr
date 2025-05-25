LEADR_BIND_KEY='{{bind_key}}'

__leadr_invoke__() {
    # === Config ===
    LEADR_CURSOR_POSITION_ENCODING="#CURSOR"
    LEADR_CMD_COLOR='\e[1;32m'
    LEADR_RESET_COLOR='\e[0m'

    leadr_parse_flags() {
        local flag_str="$1"
        local insert="" eval="false" exec="false"

        IFS='+' read -r -a flags_array <<< "$flag_str"
        for flag in "${flags_array[@]}"; do
            case "$flag" in
                REPLACE | INSERT | PREPEND | APPEND) insert="$flag" ;;
                EVAL) eval="true" ;;
                EXEC) exec="true" ;;
            esac
        done

        echo "$insert|$eval|$exec"
    }

    leadr_extract_cursor_pos() {
        local input="$1"
        local encoding="$2"
        if [[ "$input" == *"$encoding"* ]]; then
            local before="${input%%$encoding*}"
            echo "${#before}"
        else
            echo "-1"
        fi
    }

    leadr_clean_cursor_marker() {
        local input="$1"
        local encoding="$2"
        echo "${input//$encoding/}"
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
            printf "${LEADR_CMD_COLOR}%s${LEADR_RESET_COLOR}\n" "$cmd"
            history -s "$cmd"
            eval "$cmd"
        fi
    }

    leadr_main() {
        local cmd output_flags to_insert insert_type eval_flag exec_flag
        local cursor_pos

        cmd="$(leadr)"
        output_flags="${cmd%% *}"
        to_insert="${cmd#* }"

        IFS='|' read -r insert_type eval_flag exec_flag <<< "$(leadr_parse_flags "$output_flags")"

        cursor_pos="$(leadr_extract_cursor_pos "$to_insert" "$LEADR_CURSOR_POSITION_ENCODING")"
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

# === Key Binding ===
bind -x "\"${LEADR_BIND_KEY}\":__leadr_invoke__"
