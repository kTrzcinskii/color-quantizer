use std::path::Path;

use anyhow::Result;
use egui::ColorImage;
use image::ImageReader;

pub fn load_image_from_path<P: AsRef<Path>>(path: P) -> Result<ColorImage> {
    let image = ImageReader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}
