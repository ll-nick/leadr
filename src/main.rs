use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::collections::HashMap;

fn main() {
    let mut shortcuts = HashMap::new();
    shortcuts.insert("gs", ("git status", true));
    shortcuts.insert("v", ("vim ", false));
    shortcuts.insert("ll", ("ls -la", true));

    let mut sequence = String::new();

    enable_raw_mode().expect("Failed to enter raw mode");

    loop {
        if let Event::Key(KeyEvent { code, .. }) = read().expect("Failed to read event") {
            match code {
                KeyCode::Char(c) => {
                    sequence.push(c);
                    if let Some((cmd, execute_immediately)) = shortcuts.get(&sequence as &str) {
                        if *execute_immediately {
                            print!("#EXEC {}", cmd);
                        } else {
                            print!("{}", cmd); 
                        }
                        break;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");
}

