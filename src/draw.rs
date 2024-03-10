use crate::tools;

use crate::SESSION;
use crate::{color, cursor};
use crate::{CurrentScreen, ColourScheme};

struct Colours {
    bg: &'static str,
    text: &'static str,
}


pub fn draw_window()
{
    let c_s = SESSION.lock().unwrap();
    let colours: Colours;

    match c_s.colour_scheme {
        ColourScheme::LIGHT => {
            colours = Colours {
                bg: color::White.bg_str(),
                text: color::Black.fg_str(),
            };
        },
        ColourScheme::DARK => {
            colours = Colours {
                bg: color::Black.bg_str(),
                text: color::White.fg_str(),
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
        }
    }
}


fn draw_splash(colours: Colours)
{
    println!("{}{}", colours.bg, termion::clear::All);  // clear screen
    // println!("\x1b[47m"); // bg colour white
    // println!("\x1b[2J");  // clear screen

    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();

    println!("{}{}HELLO WORLD!", colours.text, cursor::Goto(x/2-6, y/2) );
}

fn draw_home(colours: Colours)
{
    println!("{}{}", colours.bg, termion::clear::All);  // clear screen

    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();

    println!("{}{}HOME!", colours.text, cursor::Goto(x/2-6, y/2) );
    tools::draw_line_v(10, 1, 10);
    tools::draw_line_h(30, 1, 61);
}
