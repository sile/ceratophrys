use crate::{Image, Render};
use std::io::Write;

#[derive(Debug)]
pub struct BmpImage {
    image: Image,
}

impl BmpImage {
    pub fn new<T: Render>(image: &T) -> Self {
        Self {
            image: image.to_image(),
        }
    }

    pub fn write_to<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        let image_data_offset: u32 = 54;
        let num_of_pixels = self.image.size().area();
        let file_size: u32 = image_data_offset + num_of_pixels * 4;

        // File header.
        writer.write_all(b"BM")?;
        writer.write_all(&file_size.to_le_bytes())?;
        writer.write_all(&[0, 0, 0, 0])?;
        writer.write_all(&image_data_offset.to_le_bytes())?;

        // Information header.
        writer.write_all(&[40, 0, 0, 0])?; // Header size.
        writer.write_all(&(self.image.size().width as i32).to_le_bytes())?;
        writer.write_all(&(self.image.size().height as i32).to_le_bytes())?;
        writer.write_all(&[1, 0])?; // Planes.
        writer.write_all(&[32, 0])?; // Bits per pixel.
        writer.write_all(&[0, 0, 0, 0])?; // No compression.
        writer.write_all(&(num_of_pixels * 4).to_le_bytes())?; // Image size.
        writer.write_all(&[0, 0, 0, 0])?; // Horizontal resolution.
        writer.write_all(&[0, 0, 0, 0])?; // Vertical resolution.
        writer.write_all(&[0, 0, 0, 0])?; // Colors in palette.
        writer.write_all(&[0, 0, 0, 0])?; // Important colors.

        // Image data.
        for pixel in self.image.pixels() {
            let (r, g, b, a) = pixel.to_rgba();
            writer.write_all(&[b, g, r, a])?;
        }
        Ok(())
    }
}
