use std::{process::Command, str};

use crate::error_handler::AppError;

pub struct EditorContents {
    contents: String,
}

// EditorContents's functions
impl EditorContents {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
        }
    }

    /// Returns the colored output which has `data` written in fg_color, on the background colored with bg_color
    pub fn get_colored_output(
        data: &str,
        fg_color: u8,
        bg_color: u8,
    ) -> Result<String, Box<AppError>> {
        /*
        Note: We specify background color after the text: `48;5`,
        and foreground color after the text: `38;5`..
        */
        let colored_string_argument =
            format!("\\033[48;5;{};38;5;{}m{}\\033[0m", bg_color, fg_color, data);

        let output_buf = Command::new("echo")
            .arg("-e")
            .arg(colored_string_argument)
            .output()
            .expect("failed to execute process")
            .stdout;

        // fetching the output_colored_string from the output buffer
        let output_colored_string = match str::from_utf8(&output_buf) {
            Ok(v) => Ok(v.to_owned()),
            Err(e) => Err(AppError::new(
                "Failed to load the image colors correctly. Please try again with other terminals..",
            )),
        };

        output_colored_string
    }
}

// EditorContents's methods
impl EditorContents {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_print_colored_hello_world() {
        /*
        Note: `95` is the code for tan color,
        and `214` is the code for light orange
        */
        let colored_output: String =
            EditorContents::get_colored_output("Hello world!", 214, 95).unwrap_or(String::new());

        println!("{}", colored_output);
    }
}
