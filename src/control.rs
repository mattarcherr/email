use crate::{SESSION, ColourScheme, CurrentScreen, PopUp};
use termion::event::Key;

pub fn switch_khit(c: Key) {
    let mut sess = SESSION.lock().unwrap();
    match sess.current_screen {

        // SPLASH Controls

        CurrentScreen::SPLASH => {

            if c == Key::Char('0') {
                match sess.popup { 
                    PopUp::None => {
                        sess.popup = PopUp::NEW_ACC;
                        std::mem::drop(sess);
                        crate::draw_window();
                    }
                    _ => {}
                }
            }
            else if c == Key::Char('b') {
                sess.current_screen = CurrentScreen::HOME;
                std::mem::drop(sess);
                crate::draw_window();
            }
            else if c == Key::Char('R') {
                sess.current_screen = CurrentScreen::RSS;
                std::mem::drop(sess);
                crate::draw_window();
            }
            else if c == Key::Char('n') {
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
            }
            else if c == Key::Char('c') {
                sess.popup = PopUp::None;
                std::mem::drop(sess);
                crate::draw_window();
            }
        },

        // HOME Controls

        CurrentScreen::HOME   => {

            if c == Key::Char('b') {
                sess.current_screen = CurrentScreen::SPLASH;
                std::mem::drop(sess);
                crate::draw_window();
            }
            else if c == Key::Char('c') {
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
            }
        }

        // RSS Controls

        CurrentScreen::RSS    => {

            if c == Key::Char('b') {
                sess.current_screen = CurrentScreen::SPLASH;
                std::mem::drop(sess);
                crate::draw_window();
            }
            else if c == Key::Char('c') {
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
            }
        }
    }
}
