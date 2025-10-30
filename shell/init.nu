def __leadr_invoke__ [] {
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

    def leadr_insert_command [to_insert:string insert_type:string exec:bool] {
        match $insert_type {
            "INSERT" => {
                if $exec {
                    commandline edit --insert --accept $to_insert
                } else {
                    commandline edit --insert $to_insert
                }
            }
            "PREPEND" => {
                let buffer = $"($to_insert)(commandline)"
                if $exec {
                    commandline edit --replace --accept $buffer
                } else {
                    commandline edit --replace $buffer
                }
            }

            "APPEND" => {
                if $exec {
                    commandline edit --append --accept $to_insert
                } else {
                    commandline edit --append $to_insert
                }
            }

            "SURROUND" => {
                let parts = ($to_insert | split row "#COMMAND")
                let buffer = $"($parts.0)(commandline)($parts.1)"

                if $exec {
                    commandline edit --replace --accept $buffer
                } else {
                    commandline edit --replace $buffer
                }
            }

            # Default is REPLACE
            _ => {
                if $exec { 
                    commandline edit --replace --accept $to_insert
                } else {
                    commandline edit --replace $to_insert
                }
            }
        }
    }

    def leadr_main [] {
        let cmd = (leadr)
        let parts = ($cmd | split row " ")
        let flags = $parts.0 | split row "+"
        let to_insert = $parts | reject 0 | str join " "

        if ($cmd | str trim | str length) == 0 {
            return
        }

        let parsed_flags = (leadr_parse_flags $flags)

        let cursor_pos = leadr_extract_cursor_pos $to_insert
        let to_insert  = ($to_insert | str replace "#CURSOR" "")

        leadr_insert_command $to_insert $parsed_flags.insert_type $parsed_flags.exec
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
