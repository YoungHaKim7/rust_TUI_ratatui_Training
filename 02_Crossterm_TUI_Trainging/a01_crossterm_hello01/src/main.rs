use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const HELP: &str = r#"Key display
 - Press any key to see its display format
 - Use Esc to quit
"#;

fn print_events() -> io::Result<()> {
    loop {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            let modifier = match modifiers {
                KeyModifiers::NONE => "".to_string(),
                _ => format!("{:?}+", modifiers),
            };
            println!("Key pressed: {}{:?}\r", modifier, code);
            if code == KeyCode::Esc {
                break;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("{}", HELP);
    enable_raw_mode()?;
    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }
    disable_raw_mode()?;
    Ok(())
}
