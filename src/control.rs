use crate::ColourScheme;
use crate::SESSION;
use crate::CurrentScreen;
use termion::event::Key;

pub fn switch_khit(c: Key) {
    let mut sess = SESSION.lock().unwrap();
    match sess.current_screen {
        CurrentScreen::SPLASH => {

            if c == Key::Char('b') {
                sess.current_screen = CurrentScreen::HOME;
                std::mem::drop(sess);
                crate::draw_window();
            }
            else if c == Key::Char('R') {
                sess.current_screen = CurrentScreen::RSS;
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
        },
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
