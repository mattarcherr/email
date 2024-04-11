mod draw;
mod control;
mod tools;
mod rss;
mod fileio;

use std::{io, io::Write, thread, sync::{Arc, Mutex}};
use termion::{color, cursor, input::TermRead, raw::IntoRawMode};
use signal_hook::{consts::SIGWINCH, iterator::Signals};
use once_cell::sync::Lazy;

use crate::{draw::draw_window, fileio::delete_account};

pub struct Session {
    current_screen: CurrentScreen,
    popup: PopUp,
    colour_scheme: ColourScheme,
    accounts: Arc<Vec<fileio::Account>>,
    selection: u8,
}

enum CurrentScreen {
    SPLASH,
    HOME,
    RSS,
}
enum PopUp {
    NONE,
    NewAcc,
    DelAcc,
}
enum ColourScheme {
    LIGHT,
    DARK,
}

static SESSION: Lazy<Mutex<Session>> = Lazy::new(
    || Mutex::new(
        Session {
            current_screen: CurrentScreen::SPLASH,
            popup: PopUp::NONE,
            colour_scheme: ColourScheme::LIGHT,
            accounts: Arc::new(fileio::read_save_file()),
            selection: 0,
        }
    )
);

fn main() -> io::Result<()> {

    println!("\x1b[?1049h"); // enter alt screen
    println!("\x1b[?25l"); // hide cursor
    // Enter raw mode
    crossterm::terminal::enable_raw_mode().unwrap();


    // Update on window size change (SIGWINCH)
    let mut signal = Signals::new(&[SIGWINCH])?;
    thread::spawn(move || {
        for _ in signal.forever() {
           draw::draw_window();
        }
    });

    draw_window();

    let mut input_events = termion::async_stdin().events();
    'main: loop {
        for event in &mut input_events {
            match event {
                Ok(termion::event::Event::Key(key)) => {
                    if key == termion::event::Key::Char('q') { break 'main; }
                    control::switch_khit(key);
                },
                Ok(termion::event::Event::Mouse(_)) => {},
                Err(error) => {},
                _ => {},
            }
        }
    }

    println!("\x1b[?25h"); // show cursor
    println!("\x1b[?1049l"); // exit alt screen
    Ok(())
}
