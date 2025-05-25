LEADR_BIND_KEY='{{bind_key}}'

__leadr_invoke__() {
    LEADR_CURSOR_POSITION_ENCODING="#CURSOR"
    LEADR_CMD_COLOR=$'\e[1;32m'
    LEADR_RESET_COLOR=$'\e[0m'

    leadr_parse_flags() {
        local flag_str="$1"
        local insert="" eval="false" exec="false"
        local flag

        IFS='+' read -A flags_array <<< "$flag_str"
        for flag in "${flags_array[@]}"; do
            case "$flag" in
                REPLACE|INSERT|PREPEND|APPEND) insert="$flag" ;;
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
            local before="${input%%${encoding}*}"
            echo "${#before}"
        else
            echo "-1"
        fi
    }

    leadr_insert_command() {
        local insert_type="$1"
        local to_insert="$2"
        local cursor_pos="$3"

        case "$insert_type" in
            INSERT)
                LBUFFER+="$to_insert"
                if [[ $cursor_pos -ge 0 ]]; then
                    CURSOR=$((CURSOR + cursor_pos))
                else
                    CURSOR=$((CURSOR + ${#to_insert}))
                fi
                ;;
            PREPEND)
                local original_cursor=$CURSOR
                BUFFER="${to_insert}${BUFFER}"
                if [[ $cursor_pos -ge 0 ]]; then
                    CURSOR=$cursor_pos
                else
                    CURSOR=$((original_cursor + ${#to_insert}))
                fi
                ;;
            APPEND)
                BUFFER="${BUFFER}${to_insert}"
                if [[ $cursor_pos -ge 0 ]]; then
                    CURSOR=$((${#BUFFER} - ${#to_insert} + cursor_pos))
                else
                    CURSOR=${#BUFFER}
                fi
                ;;
            *)
                BUFFER="$to_insert"
                if [[ $cursor_pos -ge 0 ]]; then
                    CURSOR=$cursor_pos
                else
                    CURSOR=${#BUFFER}
                fi
                ;;
        esac
    }

    leadr_execute_command() {
        local cmd="$1"
        BUFFER=""
        CURSOR=0
        if [[ -n "$TMUX" ]]; then
            tmux send-keys "$cmd" Enter
        else
            printf "${LEADR_CMD_COLOR}%s${LEADR_RESET_COLOR}\n" "$cmd"
            print -s -- "$cmd"
            eval "$cmd"
        fi
    }

    leadr_main() {
        local cmd="$(leadr)"
        local output_flags="${cmd%% *}"
        local to_insert="${cmd#* }"

        local insert_type eval_flag exec_flag
        IFS='|' read -r insert_type eval_flag exec_flag <<< "$(leadr_parse_flags "$output_flags")"

        local cursor_pos="$(leadr_extract_cursor_pos "$to_insert" "$LEADR_CURSOR_POSITION_ENCODING")"
        to_insert="${to_insert//$LEADR_CURSOR_POSITION_ENCODING/}"

        if [[ "$eval_flag" == "true" ]]; then
            to_insert="$(eval "$to_insert")"
            cursor_pos=-1
        fi

        leadr_insert_command "$insert_type" "$to_insert" "$cursor_pos"

        if [[ "$exec_flag" == "true" ]]; then
            leadr_execute_command "$BUFFER"
        fi

        zle reset-prompt
    }

    leadr_main
}

# === Key Binding ===
zle -N __leadr_invoke__
bindkey "$LEADR_BIND_KEY" __leadr_invoke__

