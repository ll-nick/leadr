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

    def leadr_main [] {
        let cmd = (leadr)
        let parts = ($cmd | split row " ")
        let flags = $parts.0 | split row "+"
        let to_insert = $parts | reject 0 | str join " "

        if ($cmd | str trim | str length) == 0 {
            return
        }

        let parsed_flags = (leadr_parse_flags $flags)
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
