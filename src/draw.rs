use crate::SESSION;
use crate::{CurrentScreen, ColourScheme};
// use crate::console::Style::Colours;
// Style::Color::Fg

pub fn draw_window() {
    let c_s = SESSION.lock().unwrap();
    // let colours: Colours;

    // match c_s.colour_scheme {
    //     ColourScheme::LIGHT => {
    //         colours = Colours {
    //             bg: color::White.bg_str(),
    //             text: color::Black.fg_str(),
    //         };
    //     },
    //     ColourScheme::DARK => {
    //         colours = Colours {
    //             bg: color::Black.bg_str(),
    //             text: color::White.fg_str(),
    //         };
    //     }
    // }

    match c_s.current_screen {
        CurrentScreen::SPLASH => {
            std::mem::drop(c_s);
            draw_splash();
        },
        CurrentScreen::HOME   => {
            std::mem::drop(c_s);
            // draw_home();
        },
        CurrentScreen::RSS    => {
            std::mem::drop(c_s);
            // draw_rss();
        }
    }
}

fn draw_splash() {
    let term = console::Term::stdout();

    println!("[107m[2J");

    let (y, x) = term.size();

    term.move_cursor_to(((x/2)-4).into(), (y/2).into()).unwrap();
    println!("[30mSPLASH");
}