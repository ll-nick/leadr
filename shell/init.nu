def __leadr_invoke__ [] {
    def leadr_cursor_line [] {
        term query (ansi cursor_position) --prefix (ansi csi) --terminator 'R' | decode utf8 | parse "{row};{col}" | get row.0
    }

    def leadr_parse_flags [flags: list<string>] {
        mut result = {
            insert_type: ""
            eval: false
            exec: false
        }
        for flag in $flags {
            if $flag == "REPLACE" or $flag == "INSERT" or $flag == "PREPEND" or $flag == "APPEND" or $flag == "SURROUND" {
                $result.insert_type = $flag
            } else if $flag == "EVAL" {
                $result.eval = true
            } else if $flag == "EXEC" {
                $result.exec = true
            }
        }

        $result
    }

    def leadr_extract_cursor_pos [input] {
        if ($input | str contains "#CURSOR") {
            let before = ($input | split row "#CURSOR" | first)
            ($before | str length)
        } else {
            -1
        }
    }

    def leadr_insert_command [to_insert:string insert_type:string cursor_pos:int] {
        let original_cursor = (commandline get-cursor)

        match $insert_type {
            "INSERT" => {
                commandline edit --insert $to_insert
                if $cursor_pos >= 0 {
                    let new_cursor = $original_cursor + $cursor_pos
                    commandline set-cursor $new_cursor
                } else {
                    let new_cursor = $original_cursor + ($to_insert | str length)
                    commandline set-cursor $new_cursor
                }
            }
            "PREPEND" => {
                let buffer = $"($to_insert)(commandline)"
                commandline edit --replace $buffer

                if $cursor_pos >= 0 {
                    let new_cursor = $cursor_pos
                    commandline set-cursor $new_cursor
                } else {
                    let new_cursor = $original_cursor + ($to_insert | str length)
                    commandline set-cursor $new_cursor
                }
            }

            "APPEND" => {
                commandline edit --append $to_insert
                if $cursor_pos >= 0 {
                    let new_cursor = (commandline | str length) - ($to_insert | str length) + $cursor_pos
                    commandline set-cursor $new_cursor
                } else {
                    commandline set-cursor --end
                }
            }

            "SURROUND" => {
                let parts = ($to_insert | parse "{before}#COMMAND{after}")
                let before = $parts.before.0
                let after = $parts.after.0
                let original_buffer = (commandline)
                let buffer = $"($before)($original_buffer)($after)"

                commandline edit --replace $buffer

                if $cursor_pos >= 0 {
                    if $cursor_pos <= ($before | str length) {
                        commandline set-cursor $cursor_pos
                    } else {
                        let new_cursor = $cursor_pos - ("#COMMAND" | str length) + ($original_buffer | str length)
                        commandline set-cursor $new_cursor
                    }
                } else {
                    let new_cursor = ($before | str length) + $original_cursor
                    commandline set-cursor $new_cursor
                }
            }

            # Default is REPLACE
            _ => {
                commandline edit --replace $to_insert

                if $cursor_pos >= 0 {
                    commandline set-cursor $cursor_pos
                } else {
                    let new_cursor = ($to_insert | str length)
                    commandline set-cursor $new_cursor
                }
            }
        }
    }

    def leadr_main [] {
        let cursor_line = leadr_cursor_line
        let cmd = (LEADR_CURSOR_LINE=$cursor_line leadr)
        if ($cmd | str trim | str length) == 0 {
            return
        }

        let parsed = ($cmd | parse "{flags} {to_insert}")

        let flags = $parsed.flags.0 | split row "+"
        let to_insert = $parsed.to_insert.0

        let parsed_flags = (leadr_parse_flags $flags)

        let cursor_pos = leadr_extract_cursor_pos $to_insert
        let to_insert  = ($to_insert | str replace "#CURSOR" "")

        leadr_insert_command $to_insert $parsed_flags.insert_type $cursor_pos

        if $parsed_flags.exec {
            commandline edit --append --accept ""
        }
    }
    leadr_main
}

$env.config.keybindings ++= [{
    name: leadr
    modifier: CONTROL
    keycode: Char_g
    mode: [emacs vi_insert vi_normal]
    event: {
        send: executehostcommand 
        cmd: "__leadr_invoke__"
    }
}]
