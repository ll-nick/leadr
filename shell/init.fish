function __leadr_invoke__
    set -g LEADR_COMMAND_POSITION_ENCODING "#COMMAND"
    set -g LEADR_CURSOR_POSITION_ENCODING "#CURSOR"

    function leadr_parse_flags
        set -l flag_str $argv[1]
        set -l insert ""
        set -l eval "false"
        set -l exec "false"

        set -l flags_array (string split '+' $flag_str)
        for flag in $flags_array
            switch $flag
                case "REPLACE" "INSERT" "PREPEND" "APPEND" "SURROUND"
                    set insert $flag
                case "EVAL"
                    set eval "true"
                case "EXEC"
                    set exec "true"
            end
        end

        echo "$insert|$eval|$exec"
    end

    function leadr_extract_cursor_pos
        set -l input $argv[1]
        if string match -q "*$LEADR_CURSOR_POSITION_ENCODING*" $input
            set -l before (string split $LEADR_CURSOR_POSITION_ENCODING $input)[1]
            echo (string length $before)
        else
            echo "-1"
        end
    end

    function leadr_insert_command
        set -l insert_type $argv[1]
        set -l to_insert $argv[2]
        set -l cursor_pos $argv[3]

        set -l original_cursor (commandline -C)

        switch $insert_type
            case "INSERT"
                commandline -i $to_insert
                if test $cursor_pos -ge 0
                    commandline -C (math $original_cursor + $cursor_pos)
                else
                    commandline -C (math $original_cursor + (string length $to_insert))
                end
            case "PREPEND"
                set -l current (commandline)
                commandline -r "$to_insert$current"
                if test $cursor_pos -ge 0
                    commandline -C $cursor_pos
                else
                    commandline -C (math $original_cursor + (string length $to_insert))
                end
            case "APPEND"
                commandline -a $to_insert
                if test $cursor_pos -ge 0
                    commandline -C (math (string length (commandline)) - (string length $to_insert) + $cursor_pos)
                else
                    commandline -C (string length (commandline))
                end
            case "SURROUND"
                set -l before (string split $LEADR_COMMAND_POSITION_ENCODING $to_insert)[1]
                set -l after (string split $LEADR_COMMAND_POSITION_ENCODING $to_insert)[2]
                set -l current (commandline)
                commandline -r "$before$current$after"

                if test $cursor_pos -ge 0
                    if test $cursor_pos -le (string length $before)
                        commandline -C $cursor_pos
                    else 
                        commandline -C (math $cursor_pos - (string length $LEADR_COMMAND_POSITION_ENCODING) + (string length $current))
                    end
                else
                    commandline -C (math (string length $before) + $original_cursor)
                end
            case "*"
                commandline -r $to_insert
                if test $cursor_pos -ge 0
                    commandline -C $cursor_pos
                else
                    commandline -C (string length (commandline))
                end
        end
    end

    function leadr_execute_command
        commandline -f execute
    end

    function leadr_main
        set -l cmd (leadr)

        test -z "$cmd"; and return

        set -l output_flags (string split ' ' $cmd)[1]
        set -l to_insert (string sub -s (math (string length $output_flags) + 2) $cmd)

        set -l parsed_flags (leadr_parse_flags $output_flags)
        set -l insert_type (string split '|' $parsed_flags)[1]
        set -l eval_flag (string split '|' $parsed_flags)[2]
        set -l exec_flag (string split '|' $parsed_flags)[3]

        set -l cursor_pos (leadr_extract_cursor_pos $to_insert)
        if test -n "$LEADR_CURSOR_POSITION_ENCODING"
            set to_insert (string replace -a $LEADR_CURSOR_POSITION_ENCODING "" $to_insert)
        end

        if test "$eval_flag" = "true"
            set to_insert (eval $to_insert)
            set cursor_pos -1
        end

        leadr_insert_command $insert_type $to_insert $cursor_pos

        if test "$exec_flag" = "true"
            leadr_execute_command
        end

        commandline -f repaint
    end

    leadr_main
end
