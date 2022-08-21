#![allow(dead_code)]
mod cleanup;
mod editor_processor;
mod error_handler;
mod input_processor;

use std::error::Error;

use cleanup::Cleanup;
use crossterm::terminal;
use editor_processor::editor::Editor;
use input_processor::args_parser::ArgParser;

fn main() -> Result<(), Box<dyn Error>> {
    let _cleanup_obj = Cleanup::new();

    // enter into the raw mode
    terminal::enable_raw_mode().expect("Unable to enter into the raw mode..");

    let mut editor = Editor::new(None);

    // parse the arguments to get the filepath if any
    if let Some(filepath) = ArgParser::parse_env_args()? {
        editor.set_file_path(filepath);

        // TODO: set the file contents as the editor's initial contents
        todo!()
    }

    // open the editor now, which listens to key inputs and prints out the contents
    editor.run()?;

    Ok(())
}
