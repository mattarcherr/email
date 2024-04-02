use std::hint::black_box;

use console::Style;

use crate::SESSION;
use crate::{CurrentScreen, ColourScheme};
// use crate::console::Colours;
// Style::Color::Fg

struct Colours {
    bg: Style,
    text: Style,
}

pub fn draw_window() {
    let c_s = SESSION.lock().unwrap();

    // Clear terminal
    print!("c");

    let colours: Colours;

    match c_s.colour_scheme {
        ColourScheme::LIGHT => {
            colours = Colours {
                bg: Style::new().on_white(),
                text: Style::new().black().on_white()
            };
        },
        ColourScheme::DARK => {
            colours = Colours {
                bg: Style::new().on_black(),
                text: Style::new().white().on_black(),
            };
        }
    }

    match c_s.current_screen {
        CurrentScreen::SPLASH => {
            std::mem::drop(c_s);
            draw_splash(colours);
        },
        CurrentScreen::HOME   => {
            std::mem::drop(c_s);
            draw_home(colours);
        },
        CurrentScreen::RSS    => {
            std::mem::drop(c_s);
            draw_rss(colours);
        }
    }
}

fn draw_splash(colours: Colours) {
    let term = console::Term::stdout();

    println!("{}", colours.bg.apply_to("[2J"));

    let (y, x) = term.size();

    term.move_cursor_to(((x/2)-4).into(), (y/2).into()).unwrap();

    println!("{}", colours.text.apply_to("SPLASH"));
}
fn draw_home(colours: Colours) {
    let term = console::Term::stdout();

    println!("[107m[2J");

    let (y, x) = term.size();

    term.move_cursor_to(((x/2)-4).into(), (y/2).into()).unwrap();
    println!("[30mHOME");
}
fn draw_rss(colours: Colours) {
    let term = console::Term::stdout();

    println!("[107m[2J");

    let (y, x) = term.size();

    term.move_cursor_to(((x/2)-4).into(), (y/2).into()).unwrap();
    println!("[30mRSS");
}