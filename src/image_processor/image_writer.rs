use std::{error::Error, process::Command, str};

use image::DynamicImage;

use crate::error_handler::AppError;

use super::image_utils::ImageUtils;

pub struct CustomImageWriter {}

// CustomImageWriter's functions
impl CustomImageWriter {
    /// Returns the colored output which has `data` written in fg_color, on the background colored with bg_color
    pub fn get_colored_output(
        data: &str,
        fg_color: u8,
        bg_color: u8,
    ) -> Result<Vec<u8>, Box<AppError>> {
        /*
        Note: We specify background color after the text: `48;5`,
        and foreground color after the text: `38;5`..
        */
        let colored_string_argument =
            format!("\\033[48;5;{};38;5;{}m{}\\033[0m", bg_color, fg_color, data);

        // `-n` is mentioned as flag, so that we can override extra new line printed by default by the `echo` command
        let output_buf = Command::new("echo")
            .arg("-en")
            .arg(colored_string_argument)
            .output()
            .expect("Failed to print image colors")
            .stdout;

        Ok(output_buf)
    }

    pub fn convert_colored_image_to_string(image: &DynamicImage) -> Result<String, Box<dyn Error>> {
        let raw_image_vector = ImageUtils::convert_dynamic_image_to_vector(image);

        let mut xterm_color_coded_vector: Vec<u8> = vec![];
        let mut resultant_colored_output_buf: Vec<u8> = vec![];

        let (len, mut i): (usize, usize) = (raw_image_vector.len(), 0);

        // TODO: do the operations on each vector using some stream, or using threads to fasten the process
        while i < len {
            let red = raw_image_vector.get(i).unwrap_or(&255);
            let green = raw_image_vector.get(i + 1).unwrap_or(&255);
            let blue = raw_image_vector.get(i + 2).unwrap_or(&255);

            // saves the xterm color code, which is created from converting the rgb to that xterm color
            xterm_color_coded_vector.push(ImageUtils::convert_rgb_to_xterm_color_code([
                *red, *green, *blue,
            ]));

            i += 3;
        }

        for &elem in xterm_color_coded_vector.iter() {
            // keeping fg_color to be 15, as it represents white in the xterm-256-color-chart
            let current_colored_output = Self::get_colored_output(" ", 15, elem)?;
            resultant_colored_output_buf.extend(current_colored_output);
        }

        ImageUtils::save_raw_image_vector_to_file(
            &resultant_colored_output_buf,
            "samples/sample_image_colored_output.txt",
        )?;

        let resultant_colored_string = ImageUtils::convert_vector_to_string(
            &resultant_colored_output_buf,
            "Failed to load the image colors correctly. Please try again with other terminals..",
        )?;

        Ok(resultant_colored_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_processor::image_reader::CustomImageReader;

    #[test]
    fn should_print_colored_image_on_terminal() {
        let image: DynamicImage =
            CustomImageReader::load_image("samples/sample_image_1.jpeg").unwrap_or_default();

        let colored_image_str =
            CustomImageWriter::convert_colored_image_to_string(&image).unwrap_or_default();

        println!("{}", colored_image_str);
    }

    #[test]
    fn should_print_linux_color_codes_with_colors() {
        for color in 0..=255 {
            let output_buf =
                CustomImageWriter::get_colored_output("Color:  ", color, color).unwrap_or_default();

            // fetching the output_colored_string from the output buffer
            let output_colored_string = ImageUtils::convert_vector_to_string(
                &output_buf,
                "Failed to load the image colors correctly. Please try again with other terminals..",
            )
            .unwrap_or_default();

            println!("{}", output_colored_string);
        }
    }
}
