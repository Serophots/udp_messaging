use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use std::io::stdout;

pub struct Terminal {}
impl Terminal {
    pub fn display_message(name: &str, msg: &str, incoming: bool) {
        if incoming {
            execute!(stdout(), Print(" -> ")).unwrap();
        } else {
            execute!(stdout(), Print(" <- ")).unwrap();
        }

        execute!(stdout(), SetForegroundColor(Color::Grey), Print("[")).unwrap();
        if incoming {
            execute!(
                stdout(),
                SetForegroundColor(Color::Cyan),
                Print(name.to_ascii_uppercase()),
            )
            .unwrap();
        } else {
            execute!(
                stdout(),
                SetForegroundColor(Color::Blue),
                Print(name.to_ascii_uppercase()),
            )
            .unwrap();
        }
        execute!(
            stdout(),
            SetForegroundColor(Color::Grey),
            Print("] "),
            SetForegroundColor(Color::White),
            Print(msg),
            Print("\n"),
            ResetColor
        )
        .unwrap();
    }
}
