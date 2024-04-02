use crate::ColourScheme;
use crate::SESSION;
use crate::CurrentScreen;

pub fn switch_khit(c: char) {
    let mut sess = SESSION.lock().unwrap();
    match sess.current_screen {
        CurrentScreen::SPLASH => {

            if c == 'b' {
                sess.current_screen = CurrentScreen::HOME;
                std::mem::drop(sess);
                crate::draw_window();
            }
            else if c == 'R' {
                sess.current_screen = CurrentScreen::RSS;
                std::mem::drop(sess);
                crate::draw_window();
            }
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
            }
        },
        CurrentScreen::HOME   => {

            if c == 'b' {
                sess.current_screen = CurrentScreen::SPLASH;
                std::mem::drop(sess);
                crate::draw_window();
            }
            // else if c == 'c' {
            //     match sess.colour_scheme {
            //         ColourScheme::DARK => {
            //             sess.colour_scheme = ColourScheme::LIGHT;
            //         },
            //         ColourScheme::LIGHT => {
            //             sess.colour_scheme = ColourScheme::DARK;
            //         }
            //     }
            //     std::mem::drop(sess);
            //     crate::draw_window();
            // }
        }
        CurrentScreen::RSS    => {

            // if c == 'b' {
            //     sess.current_screen = CurrentScreen::SPLASH;
            //     std::mem::drop(sess);
            //     crate::draw_window();
            // }
            // else if c == 'c' {
            //     match sess.colour_scheme {
            //         ColourScheme::DARK => {
            //             sess.colour_scheme = ColourScheme::LIGHT;
            //         },
            //         ColourScheme::LIGHT => {
            //             sess.colour_scheme = ColourScheme::DARK;
            //         }
            //     }
            //     std::mem::drop(sess);
            //     crate::draw_window();
            // }
        }
    }
}