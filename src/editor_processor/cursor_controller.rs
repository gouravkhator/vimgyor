use std::io::stdout;

use crossterm::{cursor, execute};

pub struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
}

// CursorController's functions
impl CursorController {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
        }
    }
}

// CursorController's methods
impl CursorController {
    pub fn update(self: &mut Self, cursor_x: usize, cursor_y: usize) -> crossterm::Result<()> {
        execute!(stdout(), cursor::MoveTo(cursor_x as u16, cursor_y as u16))?;

        self.cursor_x = cursor_x;
        self.cursor_y = cursor_y;

        Ok(())
    }

    pub fn get_current_cursor_position(self: &Self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }
}
