use image;
use image::{GenericImageView, RgbaImage, DynamicImage, RgbImage};

use crate::graphics::GraphicsError;

pub fn read_image_rgba(path: &str) -> Result<RgbaImage, GraphicsError> {
    debug!("Opening image '{}'", path);
    let img = match image::open(path.clone())? {
        image::DynamicImage::ImageRgba8(img) => img,
        _ => {
            return Err(GraphicsError::InvalidImageFormat(path.into()));
        }
    };
    Ok(img)
}

pub fn read_image_rgb(path: &str) -> Result<RgbImage, GraphicsError> {
    debug!("Opening image '{}'", path);
    let img = match image::open(path.clone())? {
        image::DynamicImage::ImageRgb8(img) => img,
        _ => {
            return Err(GraphicsError::InvalidImageFormat(path.into()));
        }
    };
    Ok(img)
}
