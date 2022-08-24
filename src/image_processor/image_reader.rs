use image::{io::Reader as ImageReader, DynamicImage};
use std::error::Error;

pub struct CustomImageReader {}

// CustomImageReader's functions
impl CustomImageReader {
    pub fn load_image(filepath: &str) -> Result<DynamicImage, Box<dyn Error>> {
        let img = ImageReader::open(filepath)?.decode()?;

        Ok(img)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_processor::image_utils::ImageUtils;

    #[test]
    fn should_save_loaded_image_to_file() {
        let sample_image_path = "samples/sample_image_2.png";

        let image: DynamicImage =
            CustomImageReader::load_image(sample_image_path).unwrap_or_default();

        let raw_image_output = ImageUtils::convert_dynamic_image_to_vector(&image);

        // save the raw image to file
        ImageUtils::save_raw_image_vector_to_file(
            &raw_image_output,
            "samples/sample_image_raw_output.txt",
        )
        .unwrap();

        // asserting that the image will be present, so Vec<u8> will not be empty
        assert_ne!(raw_image_output.len(), 0);
    }
}
