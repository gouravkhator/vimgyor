use image::DynamicImage;
use std::{error::Error, fs::File, io::Write};

use crate::error_handler::AppError;

pub struct ImageUtils {}

impl ImageUtils {
    pub fn convert_rgb_to_256_color_code(red: u8, green: u8, blue: u8) -> u8 {
        // Formula used: round(36 * (int(rgb[:2], 16) * 5) + 6 * (int(rgb[2:4], 16) * 5) + (int(rgb[4:], 16) * 5) + 16)
        let color_code = 36 * (red as u64) * 5 + 6 * (green as u64) * 5 + (blue as u64) * 5 + 16;
        color_code as u8
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
}
