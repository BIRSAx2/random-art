use crate::vec3::Vec3;
use image::{ImageBuffer, ImageError, Rgb, RgbImage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageWriteError {
    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),

    #[error("Failed to write image")]
    GenericError,
}

pub fn write_image(
    filename: &str,
    x_res: usize,
    y_res: usize,
    values: &[Vec3],
) -> Result<(), ImageWriteError> {
    let mut img: RgbImage = ImageBuffer::new(x_res as u32, y_res as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = x as usize + y as usize * x_res;
        let color = values[index];
        *pixel = Rgb([
            (color.x().clamp(0.0, 1.0) * 255.0) as u8,
            (color.y().clamp(0.0, 1.0) * 255.0) as u8,
            (color.z().clamp(0.0, 1.0) * 255.0) as u8,
        ]);
    }

    img.save(filename)?;
    Ok(())
}
