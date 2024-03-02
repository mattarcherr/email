use crate::tools;

use crate::SESSION;
use crate::{color, cursor};
use crate::CurrentScreen;

pub fn draw_window()
{
    let c_s = SESSION.lock().unwrap();
    match c_s.current_screen {
        CurrentScreen::SPLASH => {
            std::mem::drop(c_s);
            draw_splash();
        },
        CurrentScreen::HOME   => {
            std::mem::drop(c_s);
            draw_home();
        }
    }
}


fn draw_splash()
{
    println!("\x1b[47m"); // bg colour white
    println!("\x1b[2J");  // clear screen
                        


    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();


    println!("{}{}HELLO WORLD!", color::Fg(color::Red), cursor::Goto(x/2-6, y/2) );
}
fn draw_home()
{
    println!("{}{}", termion::color::Bg(color::Rgb(255,255,255)), termion::clear::All);  // clear screen

    let (x, y): (u16, u16) = termion::terminal_size().unwrap().into();

    println!("{}{}HOME!", color::Fg(color::Red), cursor::Goto(x/2-6, y/2) );
    tools::draw_line_v(10, 1, 10);
    tools::draw_line_h(30, 1, 61);
}
