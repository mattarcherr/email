use std::hint::black_box;

use console::Style;

use crate::SESSION;
use crate::{CurrentScreen, ColourScheme, PopUp};
use crate::tools::{draw_line_h, draw_line_v, draw_thick_line_h, draw_thick_line_v, draw_box, draw_thick_box, clear_area};
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
            draw_splash(&colours);
        },
        CurrentScreen::HOME   => {
            std::mem::drop(c_s);
            draw_home(&colours);
        },
        CurrentScreen::RSS    => {
            std::mem::drop(c_s);
            draw_rss(&colours);
        }
    }
        // Need to borrow new SESSION variable
    let c_s = SESSION.lock().unwrap();
    match c_s.popup {
        PopUp::NONE => {},
        PopUp::NewAcc => {
            std::mem::drop(c_s);
            popup_draw_new_acc(colours);
        }
    }
}

fn draw_splash(colours: &Colours) {
    let term = console::Term::stdout();

    println!("{}", colours.bg.apply_to("[2J")); // Clear screen

    let (y, x) = term.size();

    term.move_cursor_to(((x/2)-4).into(), (y/2).into()).unwrap();

    println!("{}", colours.text.apply_to("SPLASH"));
}
fn draw_home(colours: &Colours) {
    let term = console::Term::stdout();

    println!("{}", colours.bg.apply_to("[2J")); // Clear screen

    let (y, x) = term.size();

    term.move_cursor_to(((x/2)-4).into(), (y/2).into()).unwrap();
    println!("{}", colours.text.apply_to("HOME"));
}
fn draw_rss(colours: &Colours) {
    let term = console::Term::stdout();

    println!("{}", colours.bg.apply_to("[2J")); // Clear screen

    let (y, x) = term.size();

    let menu_line =  x/6;
    let title_line = y/6;

    draw_thick_line_h(title_line, 0, x-1, colours.text.clone());
    draw_thick_line_v(menu_line, 0, y-2,colours.text.clone());

    let x_padding = x/14;
    let box_height = 5;
    let box_width = x-menu_line-(x_padding*2-2);

    let mut i = 0;
    loop {
        let box_y = (i*6)+title_line+2;
        let box_x = menu_line+x_padding;

        if box_y >= y-5 { break; }

        draw_box(box_x, box_y, box_width, box_height, colours.text.clone());

        term.move_cursor_to((box_x+1).into(), (box_y+1).into()).unwrap();
        println!("{}", colours.text.apply_to("BBC UK"));
        term.move_cursor_to((box_x+box_width-8).into(), (box_y+1).into()).unwrap();
        println!("{}", colours.text.apply_to("22/05/24"));

        i += 1;
        break;
    }
}

fn popup_draw_new_acc(colours: Colours) {
    let term = console::Term::stdout();
    let (y, x) = term.size();

    let sess = SESSION.lock().unwrap();

    let xpos = x/8; 
    let ypos = y/5; // Top left of popup (x/8, y/5)
    let width = x-(x*2/8);
    let height = y-(y*2/5);

    draw_thick_box(xpos, ypos, width, height, colours.text.clone());
    clear_area(xpos+1, ypos+1, width-1, height-1, colours.bg);

    term.move_cursor_to((xpos+(width*2/4)-5).into(), (ypos+(height/5)-2).into()).unwrap();
    println!("{}", colours.text.clone().apply_to("Account name:"));
    if sess.selection == 1 { 
        draw_line_h(ypos+(height/5), xpos+(width*1/4), xpos+(width*3/4), Style::new().red());
    } else {
        draw_line_h(ypos+(height/5), xpos+(width*1/4), xpos+(width*3/4), colours.text.clone());
    }
    term.move_cursor_to((xpos+(width*2/4)-6).into(), (ypos+(height/3)-2).into()).unwrap();
    println!("{}", colours.text.clone().apply_to("Account email:"));
    if sess.selection == 2 { 
        draw_line_h(ypos+(height/3), xpos+(width*1/4), xpos+(width*3/4), Style::new().red());
    } else { 
        draw_line_h(ypos+(height/3), xpos+(width*1/4), xpos+(width*3/4), colours.text.clone());
    }
    
    if sess.selection == 3 { 
        draw_box(xpos+(width*1/4), ypos+(height*3/5), width/2, 2, Style::new().red());
    } else { 
        draw_box(xpos+(width*1/4), ypos+(height*3/5), width/2, 2, colours.text.clone());
    }
    term.move_cursor_to((xpos+(width*2/4)-9).into(), (ypos+(height*3/5)+1).into()).unwrap();
    println!("{}", colours.text.clone().apply_to("Create new account"));

    if sess.selection == 4 {
        draw_box(xpos+(width*1/4), ypos+(height*4/5), width/2, 2, Style::new().red());
    } else {
        draw_box(xpos+(width*1/4), ypos+(height*4/5), width/2, 2, colours.text.clone());
    }
    term.move_cursor_to((xpos+(width*2/4)-7).into(), (ypos+(height*4/5)+1).into()).unwrap();
    println!("{}", colours.text.clone().apply_to("Delete account"));

    std::mem::drop(sess);
}
