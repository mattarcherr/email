use crate::{ColourScheme, CurrentScreen, PopUp};
use crate::SESSION;

pub fn switch_khit(c: char) {
    let mut sess = SESSION.lock().unwrap();

    if c == 'q' { return }
    else if c == 'c' {
        match sess.colour_scheme {
            ColourScheme::DARK => {
                sess.colour_scheme = ColourScheme::LIGHT;
            },
            ColourScheme::LIGHT => {
                sess.colour_scheme = ColourScheme::DARK;
            }
        }
        std::mem::drop(sess);
        crate::draw_window();
    } else {

        match sess.current_screen {
            CurrentScreen::SPLASH => {

                if c == '0' {
                    match sess.popup { 
                        PopUp::NONE => {
                            sess.popup = PopUp::NewAcc;
                            std::mem::drop(sess);
                            crate::draw_window();
                        }
                        _ => {}
                    }
                }
                else if c == 'b' {
                    sess.current_screen = CurrentScreen::HOME;
                    std::mem::drop(sess);
                    crate::draw_window();
                }
                else if c == 'R' {
                    sess.current_screen = CurrentScreen::RSS;
                    std::mem::drop(sess);
                    crate::draw_window();
                }

            },
            CurrentScreen::HOME   => {

                if c == 'b' {
                    sess.current_screen = CurrentScreen::SPLASH;
                    std::mem::drop(sess);
                    crate::draw_window();
                }
            }
            CurrentScreen::RSS    => {

                if c == 'b' {
                    sess.current_screen = CurrentScreen::SPLASH;
                    std::mem::drop(sess);
                    crate::draw_window();
                }
            }
        }
    }
}