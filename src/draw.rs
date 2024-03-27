use crate::tools::{draw_line_h, draw_line_v, draw_thick_line_h, draw_thick_line_v, draw_box};

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
            draw_splash(colours);
        },
        CurrentScreen::HOME   => {
            draw_home(colours);
        },
        CurrentScreen::RSS    => {
            draw_rss(colours);
        }
    }

    std::mem::drop(c_s);
}


fn draw_splash(colours: Colours)
{
    println!("{}{}", colours.bg, termion::clear::All);  // clear screen

    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();

    println!("{}{}VIMAIL", colours.text, cursor::Goto(x/2-3, y/2) );


    println!("{}{}0 - Create new account", colours.text, cursor::Goto(x/2-11, y/2+5) );
}

fn draw_home(colours: Colours)
{
    println!("{}{}", colours.bg, termion::clear::All);  // clear screen

    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();

    println!("{}{}HOME!", colours.text, cursor::Goto(x/2-6, y/2) );
    draw_line_v(10, 1, 10);
    draw_line_h(30, 1, 61);
}

fn draw_rss(colours: Colours)
{
    println!("{}{}", colours.bg, termion::clear::All);  // clear screen

    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();

    let menu_line =  x/6;
    let title_line = y/6;

    draw_thick_line_h(title_line, 0, x+1); 
    draw_thick_line_v(menu_line, 1, y-1); 

    let x_padding = x/14;
    let box_height = 5;
    let box_width = x-menu_line-(x_padding*2-2);

    let mut i = 0;
    loop {
        let box_y = (i*6)+title_line+2;
        let box_x = menu_line+x_padding;

        if box_y >= y-5 { break; }

        draw_box(box_x, box_y, box_width, box_height);

        println!("{}{}{}", colours.text, cursor::Goto(box_x+1, box_y+1), "BBC UK");
        println!("{}{}{}", colours.text, cursor::Goto(box_x+box_width-8, box_y+1), "22/05/24");

        i += 1;
        break;
    }
}
