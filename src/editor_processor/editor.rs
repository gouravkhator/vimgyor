use std::io::stdout;

use crossterm::{
    execute,
    terminal::{self, ClearType},
};

use super::{
    cursor_controller::CursorController, editor_contents::EditorContents, editor_modes::EditorMode,
};
use crate::input_processor::keypress_handler::KeypressHandler;

pub struct Editor {
    window_size: (usize, usize),
    filepath: Option<String>,
    mode: EditorMode,
    cursor_controller: CursorController,
    keypress_handler: KeypressHandler,
    contents: EditorContents,
}

// Editor's functions
impl Editor {
    pub fn new(filepath: Option<String>) -> Self {
        let window_size = terminal::size()
            .map(|(width, height)| (width as usize, height as usize))
            .unwrap();

        Self {
            window_size,
            filepath,
            mode: EditorMode::COMMAND,
            keypress_handler: KeypressHandler::new(),
            cursor_controller: CursorController::new(),
            contents: EditorContents {},
        }
    }
}

// Editor's methods
impl Editor {
    pub fn run(self: &mut Self) -> crossterm::Result<()> {
        // clear the editor
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        self.cursor_controller.update(0, 0)?;

        // TODO: have the screen drawn initially with the file contents or the new editor window

        // the keypress handler listens for keypress events
        self.keypress_handler
            .listen_key_presses(&mut self.cursor_controller)?;
        Ok(())
    }
}

// setter and getter methods if needed
impl Editor {
    pub fn set_file_path(self: &mut Self, filepath: String) {
        self.filepath = Some(filepath);
    }
}
