use crate::control;

use super::verrors::VError;

use std::{panic, io::{stdout, Write}};
use crossterm::{cursor::{Hide, SavePosition, Show}, event::{Event, KeyEvent, KeyEventKind}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};


pub fn launch() -> Result<(), VError>{
    //If the main function causes panic, catch it.
    let result = panic::catch_unwind(|| run());
    // leave_raw_mode();
    execute!(stdout(), LeaveAlternateScreen)?;
    print!("{}", Show); // Show Cursor
    crossterm::terminal::disable_raw_mode().ok(); // exit raw mode


    result.ok().unwrap()
}
fn run() -> Result<(), VError>{

    // Setup terminal
    let mut screen = stdout();
    write!(screen, "{}", SavePosition)?; // Save cursor pos
    crossterm::terminal::enable_raw_mode().ok(); // enter raw mode
    print!("{}", Hide); // Hide Cursor
    execute!(screen, EnterAlternateScreen)?; // enter alt screen

    let mut context: crate::context::Context = crate::context::Context {  };
    context.draw();

    // Main loop
    'main: loop {
        match crossterm::event::read()? {

            Event::Key(event) => { if let Err(_) = control::event_key(event) { break 'main } },
            Event::Mouse(event) => {},
            Event::Resize(width, height) => {},
            _ => { break 'main },
        }
    }

    Ok(())
}
