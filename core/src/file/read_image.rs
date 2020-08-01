use image;
use image::{GenericImageView, RgbaImage};

use crate::graphics::GraphicsError;

pub fn read_image(path: &str) -> Result<RgbaImage, GraphicsError> {
    debug!("Opening image '{}'", path);
    let img = match image::open(path.clone())? {
        image::DynamicImage::ImageRgba8(img) => img,
        _ => {
            return Err(GraphicsError::InvalidImageFormat(path.into()));
        }
    };
    Ok(img)
}
