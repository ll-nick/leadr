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

    def leadr_insert_command [to_insert:string insert_type:string exec:bool cursor_pos:int] {
        let original_cursor = (commandline get-cursor)

        match $insert_type {
            "INSERT" => {
                if $exec {
                    commandline edit --insert --accept $to_insert
                } else {
                    commandline edit --insert $to_insert
                    if $cursor_pos >= 0 {
                        let new_cursor = $original_cursor + $cursor_pos
                        commandline set-cursor $new_cursor
                    } else {
                        let new_cursor = $original_cursor + ($to_insert | str length)
                        commandline set-cursor $new_cursor
                    }
                }
            }
            "PREPEND" => {
                let buffer = $"($to_insert)(commandline)"
                if $exec {
                    commandline edit --replace --accept $buffer
                } else {
                    commandline edit --replace $buffer

                    if $cursor_pos >= 0 {
                        let new_cursor = $cursor_pos
                        commandline set-cursor $new_cursor
                    } else {
                        let new_cursor = $original_cursor + ($to_insert | str length)
                        commandline set-cursor $new_cursor
                    }
                }
            }

            "APPEND" => {
                if $exec {
                    commandline edit --append --accept $to_insert
                } else {
                    commandline edit --append $to_insert
                    if $cursor_pos >= 0 {
                        let new_cursor = (commandline | str length) - ($to_insert | str length) + $cursor_pos
                        commandline set-cursor $new_cursor
                    } else {
                        commandline set-cursor --end
                    }
                }
            }

            "SURROUND" => {
                let parts = ($to_insert | parse "{before}#COMMAND{after}")
                let before = $parts.before.0
                let after = $parts.after.0
                let original_buffer = (commandline)
                let buffer = $"($before)($original_buffer)($after)"

                if $exec {
                    commandline edit --replace --accept $buffer
                } else {
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
            }

            # Default is REPLACE
            _ => {
                if $exec { 
                    commandline edit --replace --accept $to_insert
                } else {
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
    }

    def leadr_main [] {
        let cursor_line = leadr_cursor_line
        let cmd = (LEADR_CURSOR_LINE=$cursor_line leadr)

        let parts = ($cmd | split row " ")
        let flags = $parts.0 | split row "+"
        let to_insert = $parts | reject 0 | str join " "

        if ($cmd | str trim | str length) == 0 {
            return
        }

        let parsed_flags = (leadr_parse_flags $flags)

        let cursor_pos = leadr_extract_cursor_pos $to_insert
        let to_insert  = ($to_insert | str replace "#CURSOR" "")

        leadr_insert_command $to_insert $parsed_flags.insert_type $parsed_flags.exec $cursor_pos
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
