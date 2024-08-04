use std::fs::File;
use std::io::{Write, BufWriter};
use crate::color::Color;

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

pub fn write_bmp_file(
    file_path: &str, 
    buffer: &[Color], 
    width: usize, 
    height: usize
) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    writer: &mut BufWriter<File>, 
    width: usize, 
    height: usize
) -> std::io::Result<()> {
    let file_size = BMP_HEADER_SIZE + (width * height * BMP_BITS_PER_PIXEL / 8);
    let pixel_data_size = width * height * BMP_BITS_PER_PIXEL / 8;

    // BMP Header
    writer.write_all(b"BM")?; // Signature
    writer.write_all(&(file_size as u32).to_le_bytes())?; // File size
    writer.write_all(&[0; 4])?; // Reserved
    writer.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?; // Pixel data offset

    // DIB Header
    writer.write_all(&(40 as u32).to_le_bytes())?; // Header size
    writer.write_all(&(width as i32).to_le_bytes())?; // Width
    writer.write_all(&(height as i32).to_le_bytes())?; // Height
    writer.write_all(&(1 as u16).to_le_bytes())?; // Color planes
    writer.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?; // Bits per pixel
    writer.write_all(&(0 as u32).to_le_bytes())?; // Compression method
    writer.write_all(&(pixel_data_size as u32).to_le_bytes())?; // Pixel data size
    writer.write_all(&(2835 as u32).to_le_bytes())?; // Horizontal resolution (pixels/meter)
    writer.write_all(&(2835 as u32).to_le_bytes())?; // Vertical resolution (pixels/meter)
    writer.write_all(&(0 as u32).to_le_bytes())?; // Number of colors in palette
    writer.write_all(&(0 as u32).to_le_bytes())?; // Important colors

    Ok(())
}

fn write_pixel_data(
    writer: &mut BufWriter<File>, 
    buffer: &[Color], 
    width: usize, 
    height: usize
) -> std::io::Result<()> {
    let padding_size = (4 - (width * 3 % 4)) % 4;
    let padding = [0u8; 3];

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            writer.write_all(&[pixel.b, pixel.g, pixel.r])?;
        }
        writer.write_all(&padding[..padding_size])?;
    }

    Ok(())
}
