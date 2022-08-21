use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::editor_processor::cursor_controller::CursorController;

pub struct KeypressHandler;

impl KeypressHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl KeypressHandler {
    fn read_key_press() -> crossterm::Result<Option<KeyEvent>> {
        if let Event::Key(key_event) = event::read()? {
            return Ok(Some(key_event));
        }

        return Ok(None);
    }

    pub fn listen_key_presses(
        self: &Self,
        cursor_controller: &mut CursorController,
    ) -> crossterm::Result<()> {
        loop {
            if let Some(key_event) = Self::read_key_press()? {
                match key_event {
                    KeyEvent {
                        code: KeyCode::Char(_c @ ('q' | 'z')),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        return Ok(());
                    }

                    KeyEvent {
                        code: KeyCode::Char(cursor_direction @ ('h' | 'j' | 'k' | 'l')),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        // TODO: if the mode is a command mode,
                        // then validate the cursor new position with the window size, and then only move it to new position..
                        println!("{:?}\r", key_event);
                    }

                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::terminal;

    #[test]
    fn test() {
        terminal::enable_raw_mode().expect("Unable to enter into the raw mode..");
        KeypressHandler::new()
            .listen_key_presses(&mut CursorController::new())
            .unwrap();
    }
}
