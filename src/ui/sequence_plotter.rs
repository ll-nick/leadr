use std::io::Write;

use crossterm::{QueueableCommand, cursor, terminal};

pub struct SequencePlotter {
    pub print_sequence: bool,
    pub padding: usize,
}

impl SequencePlotter {
    pub fn new(print_sequence: bool, padding: usize) -> Self {
        SequencePlotter {
            print_sequence,
            padding,
        }
    }

    pub fn update(&self, sequence: &str) -> std::io::Result<()> {
        if self.print_sequence {
            self.print_sequence_bottom_right(sequence)?;
        }
        Ok(())
    }

    /// Displays the current input sequence at the bottom right of the terminal.
    fn print_sequence_bottom_right(&self, sequence: &str) -> std::io::Result<()> {
        let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;

        let (cols, rows) = terminal::size().unwrap_or((80, 24));
        let max_len = sequence.len().min(cols as usize);
        let x = cols.saturating_sub((max_len + self.padding) as u16);
        let y = rows.saturating_sub(1);

        stdout
            .queue(cursor::SavePosition)?
            .queue(cursor::MoveTo(x, y))?
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
            .queue(cursor::Hide)?;

        write!(stdout, "{}", &sequence[..max_len])?;

        stdout
            .queue(cursor::Show)?
            .queue(cursor::RestorePosition)?
            .flush()?;

        Ok(())
    }
}

pub fn clear_bottom_line() -> std::io::Result<()> {
    let mut stdout = std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
    let (_cols, rows) = terminal::size().unwrap_or((80, 24));
    let y = rows.saturating_sub(1);

    stdout
        .queue(cursor::SavePosition)?
        .queue(cursor::MoveTo(0, y))?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
        .queue(cursor::RestorePosition)?
        .flush()?;

    Ok(())
}

impl Drop for SequencePlotter {
    fn drop(&mut self) {
        let _ = clear_bottom_line();
    }
}
