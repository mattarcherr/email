mod draw;
mod control;
mod tools;
mod fileio;

use console::Term;

use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

use crate::draw::draw_window;

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

fn main() {
    let term = Term::stdout();
    term.set_title("Vimail");

    draw::draw_window();

    loop {
        let c = term.read_char().unwrap();
        if c == 'q' {
            println!("[0m    [2J");
            break;
        }
        else { control::switch_khit(c) }
    }

}
