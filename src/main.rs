mod draw;
mod control;
mod tools;

use console::Term;

use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::draw::draw_window;

pub struct Session {
    current_screen: CurrentScreen,
    colour_scheme: ColourScheme,
}
enum CurrentScreen {
    SPLASH,
    HOME,
    RSS,
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
