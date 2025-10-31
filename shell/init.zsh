__leadr_invoke__() {
    LEADR_COMMAND_POSITION_ENCODING="#COMMAND"
    LEADR_CURSOR_POSITION_ENCODING="#CURSOR"

    # Reference: https://stackoverflow.com/a/43911767
    leadr_cursor_line() {
      echo -ne "\033[6n" > /dev/tty
      read -t 1 -s -d 'R' line < /dev/tty
      line="${line##*\[}"
      line="${line%;*}"
      echo $((line - 1))
    }

    leadr_parse_flags() {
        local flag_str="$1"

        local insert=""
        local eval="false"
        local exec="false"
        local flag

        IFS='+' read -A flags_array <<< "$flag_str"
        for flag in "${flags_array[@]}"; do
            case "$flag" in
                REPLACE|INSERT|PREPEND|APPEND|SURROUND) insert="$flag" ;;
                EVAL) eval="true" ;;
                EXEC) exec="true" ;;
            esac
        done

        echo "$insert|$eval|$exec"
    }

    leadr_extract_cursor_pos() {
        local input="$1"

        if [[ "$input" == *"$LEADR_CURSOR_POSITION_ENCODING"* ]]; then
            local before="${input%%${LEADR_CURSOR_POSITION_ENCODING}*}"
            echo "${#before}"
        else
            echo "-1"
        fi
    }

    leadr_insert_command() {
        local insert_type="$1"
        local to_insert="$2"
        local cursor_pos="$3"

        local original_cursor=$CURSOR

        case "$insert_type" in
            INSERT)
                LBUFFER+="$to_insert"
                if [[ $cursor_pos -ge 0 ]]; then
                    CURSOR=$((original_cursor + cursor_pos))
                else
                    CURSOR=$((original_cursor + ${#to_insert}))
                fi
                ;;
            PREPEND)
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
            SURROUND)
                local before="${to_insert%%$LEADR_COMMAND_POSITION_ENCODING*}"
                local after="${to_insert#*$LEADR_COMMAND_POSITION_ENCODING}"
                local original_buffer="$BUFFER"
                BUFFER="${before}${BUFFER}${after}"

                if [[ $cursor_pos -ge 0 ]]; then
                    if [[ $cursor_pos -le ${#before} ]]; then
                        CURSOR=$cursor_pos
                    else
                        CURSOR=$(($cursor_pos - ${#LEADR_COMMAND_POSITION_ENCODING} + ${#original_buffer}))
                    fi
                else
                    CURSOR=$((${#before} + original_cursor))
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
            printf "%s\n" "$cmd"
            print -s -- "$cmd"
            eval "$cmd"
        fi
    }

    leadr_main() {
        local cmd="$(LEADR_CURSOR_LINE=$(leadr_cursor_line) leadr)"

        [[ -z "$cmd" ]] && return

        local output_flags="${cmd%% *}"
        local to_insert="${cmd#* }"

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
            leadr_execute_command "$BUFFER"
        fi

        zle reset-prompt
    }

    leadr_main
}
