use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};

pub fn event_key(event: KeyEvent) -> Result<(),()>{
        match event.modifiers {
            KeyModifiers::NONE => match event.code {
                KeyCode::Char('q') => {
                    return Err(())
                }
                _ => {},
            },
            _ => {},
        }
        Ok(())
}
