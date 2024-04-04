use crate::{Arc, fileio, ColourScheme, CurrentScreen, PopUp, SESSION};
use signal_hook::low_level::exit;
use termion::event::Key;

pub fn switch_khit(c: Key) {
    let mut sess = SESSION.lock().unwrap();

    match sess.popup {
        // No popup
        PopUp::NONE => {
            match sess.current_screen {

                // SPLASH Controls

                CurrentScreen::SPLASH => {

                    if c == Key::Char('0') {
                        match sess.popup { 
                            PopUp::NONE => {
                                sess.popup = PopUp::NewAcc;
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
        },
        // Popup controls
        PopUp::NewAcc => {
            // println!("{:#?}", c);
            if c == Key::Char('\t') {
                if sess.accounts.len() > 0 {
                    if sess.selection >= 4 { sess.selection = 0; }
                    else { sess.selection += 1; }
                }
                else {
                    if sess.selection >= 3 { sess.selection = 0; }
                    else { sess.selection += 1; }
                }
                std::mem::drop(sess);
                crate::draw_window();
            } 
            else if c == Key::Char('\n') {
                if sess.selection == 3 {}
                if sess.selection == 4 {
                    sess.popup = PopUp::DelAcc;
                    std::mem::drop(sess);
                    crate::draw_window();
                } 
            }
            else if c == Key::Char('c') {
                sess.popup = PopUp::NONE;
                 std::mem::drop(sess);
                 crate::draw_window();
            }
        },
        PopUp::DelAcc => {
            if c == Key::Char('\t') {
                if sess.selection >= sess.accounts.len() as u8 { sess.selection = 0; }
                else { sess.selection += 1; }
                std::mem::drop(sess);
                crate::draw_window();
            } 
            else if c == Key::Char('\n') {
                if sess.selection <= sess.accounts.len() as u8 {
                    sess.popup = PopUp::NONE;
                    std::mem::drop(sess);
                    crate::delete_account();
                    // SESSION.lock().unwrap().accounts = Arc::new(fileio::read_save_file());
                    crate::draw_window();
                }
            }
            else if c == Key::Char('c') {
                sess.popup = PopUp::NONE;
                 std::mem::drop(sess);
                 crate::draw_window();
            }
        },
    }
}
