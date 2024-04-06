mod draw;
mod control;
mod tools;
mod rss;
mod fileio;

use std::{io, thread, sync::{Arc, Mutex}};
use termion::{color, cursor, input::TermRead};
use signal_hook::{consts::SIGWINCH, iterator::Signals};

use once_cell::sync::Lazy;

use crate::draw::draw_window;
use crate::fileio::delete_account;

// extern crate termion;


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
    // let mut stdout = stdout().into_raw_mode()?;
    // write!(stdout, "").unwrap();
    // stdout.flush().unwrap();


    // Update on window size change (SIGWINCH)
    let mut signal = Signals::new(&[SIGWINCH])?;
    thread::spawn(move || {
        for _ in signal.forever() {
           draw::draw_window();
        }
    });

    draw_window();

    let mut it = termion::async_stdin().keys();
    loop {
        let b = it.next();
        match b {
            Some(x) => match x {
                Ok(k) => {
                    if k == termion::event::Key::Char('q') {
                        break;
                    }
                    control::switch_khit(k);
                }
                _ => {}
            },
            None => {}
        }
    }

    println!("\x1b[?25h"); // show cursor
    println!("\x1b[?1049l"); // exit alt screen
    Ok(())
}
