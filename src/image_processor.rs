use std::error::Error;

use image::{io::Reader as ImageReader, DynamicImage};

pub struct ImageProcessor {}

impl ImageProcessor {
    fn load_image(filepath: String) -> Result<DynamicImage, Box<dyn Error>> {
        let img = ImageReader::open(filepath)?.decode()?;

        Ok(img)
    }
}
