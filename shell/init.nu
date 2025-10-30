def __leadr_invoke__ [] {
    def leadr_main [] {
        let cmd = (leadr)
        let parts = ($cmd | split row " ")
        let output_flags = $parts.0
        let to_insert = $parts | reject 0 | str join " "

        if ($cmd | str trim | str length) == 0 {
            return
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
