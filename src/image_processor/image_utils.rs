use ansi_colours;
use image::DynamicImage;
use std::{error::Error, fs::File, io::Write};

use crate::error_handler::AppError;

pub struct ImageUtils {}

impl ImageUtils {
    pub fn convert_rgb_to_xterm_color_code(rgb: [u8; 3]) -> u8 {
        ansi_colours::ansi256_from_rgb(rgb)
    }

    pub fn convert_dynamic_image_to_vector(image: &DynamicImage) -> Vec<u8> {
        let rgb_image_output = image.to_rgb8();
        let raw_image_output = rgb_image_output.into_raw();

        raw_image_output
    }

    pub fn convert_vector_to_string(
        buf: &Vec<u8>,
        error_msg_if_failed: &str,
    ) -> Result<String, Box<AppError>> {
        let output_colored_string = match std::str::from_utf8(&buf) {
            Ok(v) => Ok(v.to_owned()),
            Err(_) => Err(AppError::new(error_msg_if_failed)),
        };

        output_colored_string
    }

    pub fn save_raw_image_vector_to_file(
        raw_image_vector: &Vec<u8>,
        output_file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut raw_image_in_str: String = String::new();

        for elem in raw_image_vector.iter() {
            raw_image_in_str.push_str(&format!("{}, ", *elem));
        }

        let mut file = File::create(output_file_path)?;
        file.write_all(raw_image_in_str.as_bytes())?;
        Ok(())
    }

    // this function is an experimental function, which will be explored later if needed
    pub fn unimplemented_get_closest_xterm_color_code(rgb: [u8; 3]) -> u8 {
        let incs = [0, 95, 135, 175, 215, 255];
        let (mut res, mut closest) = (vec![], 0);

        for &part in rgb.iter() {
            let mut i = 0;

            while i < incs.len() - 1 {
                let (smaller, bigger) = (incs[i], incs[i + 1]);

                if smaller <= part && part <= bigger {
                    let (s1, b1) = (smaller.abs_diff(part), bigger.abs_diff(part));

                    if s1 < b1 {
                        closest = smaller;
                    } else {
                        closest = bigger;
                    }

                    res.push(closest);
                    break;
                }

                i += 1;
            }
        }

        /*
        res contains the closest triplet to the inputted rgb,
        but I am unsure know as to how to convert this `res` to xterm 256-color code
        */
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_print_rgb_as_xterm_color_code() {
        assert_eq!(
            215,
            ImageUtils::convert_rgb_to_xterm_color_code([241, 165, 52])
        );
        assert_eq!(16, ImageUtils::convert_rgb_to_xterm_color_code([1, 1, 1]));
        assert_eq!(16, ImageUtils::convert_rgb_to_xterm_color_code([0, 1, 2]));
        assert_eq!(
            67,
            ImageUtils::convert_rgb_to_xterm_color_code([95, 135, 175])
        );
        assert_eq!(
            231,
            ImageUtils::convert_rgb_to_xterm_color_code([255, 255, 255])
        );
    }
}
