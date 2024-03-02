mod draw;
mod control;

use std::io::{self, Write, stdout};
use termion::input::TermRead;
use termion::color;
use termion::cursor;
use termion::raw::IntoRawMode;
use signal_hook::consts::SIGWINCH;
use signal_hook::iterator::Signals;
use std::thread;

use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::draw::draw_window;

extern crate termion;


pub struct Session {
    current_screen: CurrentScreen,
    colour_scheme: ColourScheme,
}

enum CurrentScreen {
    SPLASH,
    HOME,
}
enum ColourScheme {
    LIGHT,
    DARK,
}

static SESSION: Lazy<Mutex<Session>> = Lazy::new(
    || Mutex::new(
        Session {
            current_screen: CurrentScreen::SPLASH,
            colour_scheme: ColourScheme::LIGHT,
        }
    )
);

fn main() -> io::Result<()> {
    // let stdin = std::io::stdin();
    // let mut stdin = stdin.lock();


    println!("\x1b[?1049h"); // enter alt screen
    println!("\x1b[?25l"); // hide cursor
    // Enter raw mode
    let mut stdout = stdout().into_raw_mode()?;
    write!(stdout, "").unwrap();
    stdout.flush().unwrap();


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
