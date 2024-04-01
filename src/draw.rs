use termion::clear;
use termion::cursor::Goto;

use crate::tools::{draw_line_h, draw_line_v, draw_thick_line_h, draw_thick_line_v, draw_box, draw_thick_box, clear_area};

use std::sync::Arc;
use crate::SESSION;
use crate::{color, cursor};
use crate::{CurrentScreen, ColourScheme, PopUp};

#[derive(Copy, Clone)]
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
        },
        CurrentScreen::RSS    => {
            std::mem::drop(c_s);
            draw_rss(colours);
        }
    }

    // Need to borrow new SESSION variable
    let c_s = SESSION.lock().unwrap();
    match c_s.popup {
        PopUp::NONE => {},
        PopUp::NEW_ACC => {
            std::mem::drop(c_s);
            popup_draw_new_acc(colours);
        }
    }
}


fn draw_splash(colours: Colours)
{
    println!("{}{}", colours.bg, termion::clear::All);  // clear screen

    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();

    println!("{}{}VIMAIL", colours.text, cursor::Goto(x/2-3, y/2) );

    
    let accounts = Arc::clone(&SESSION.lock().unwrap().accounts);


    let mut i = 1;
    for account in accounts.iter() {
        let offset: u16 = account.name.len() as u16;
        println!("{}{}{i} - {}", colours.text, cursor::Goto(x/2-offset, y/2+2+i), account.name );
        i += 1;
    }
    println!("{}{}0 - Create new account", colours.text, cursor::Goto(x/2-11, y/2+2+i) );
    std::mem::drop(accounts);
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

fn popup_draw_new_acc(colours: Colours) {
    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();
    println!("{}{}{x},{y}", colours.text, cursor::Goto(x/2-6, y/2) );

    let sess = SESSION.lock().unwrap();

    let xpos = x/8; 
    let ypos = y/5; // Top left of popup (x/8, y/5)
    let width = x-(x*2/8);
    let height = y-(y*2/5);

    draw_thick_box(xpos, ypos, width, height);
    clear_area(xpos+1, ypos+1, width-1, height-1, colours.bg);

    println!("{}{}Account name:", colours.text, cursor::Goto(xpos+(width*2/4)-5, ypos+(height/5)-2));
    if sess.selection == 1 { println!("{}", color::Red.fg_str()); } else { println!("{}", colours.text); }
    draw_line_h(ypos+(height/5), xpos+(width*1/4), xpos+(width*3/4));
    println!("{}{}Account email:", colours.text, cursor::Goto(xpos+(width*2/4)-6, ypos+(height/3)-2));
    if sess.selection == 2 { println!("{}", color::Red.fg_str()); } else { println!("{}", colours.text); }
    draw_line_h(ypos+(height/3), xpos+(width*1/4), xpos+(width*3/4));

    if sess.selection == 3 { println!("{}", color::Red.fg_str()); } else { println!("{}", colours.text); }
    draw_box(xpos+(width*1/4), ypos+(height*3/5), width/2, 2);
    println!("{}{}Create new account", colours.text, cursor::Goto(xpos+(width*2/4)-9, ypos+(height*3/5)+1));
    if sess.selection == 4 { println!("{}", color::Red.fg_str()); } else { println!("{}", colours.text); }
    draw_box(xpos+(width*1/4), ypos+(height*4/5), width/2, 2);
    println!("{}{}Delete account", colours.text, cursor::Goto(xpos+(width*2/4)-7, ypos+(height*4/5)+1));
    std::mem::drop(sess);
}
