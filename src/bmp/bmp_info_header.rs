use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

/// Builds a `BmpInfoHeader` struct from a file.
///
/// # Arguments
///
/// * `file` - A mutable reference to a `File` object.
///
/// # Returns
///
/// Returns a `Result` containing the `BmpInfoHeader` if successful, or an `io::Error` if an error occurred.

#[repr(C)]
pub struct BmpInfoHeader {
    pub size: [u8; 4],
    pub width: [u8; 4],
    pub height: [u8; 4],
    pub planes: [u8; 2],
    pub bits_per_px: [u8; 2],
    pub compression: [u8; 4],
    pub image_size: [u8; 4],
    pub x_per_m: [u8; 4],
    pub y_per_m: [u8; 4],
    pub colours_used: [u8; 4],
    pub important_colours: [u8; 4]
}

impl BmpInfoHeader {
    pub fn build_from_file(file: &mut File) -> io::Result<Self> {
        let mut info_header = BmpInfoHeader {
            size: [0; 4],
            width: [0; 4],
            height: [0; 4],
            planes: [0; 2],
            bits_per_px: [0; 2],
            compression: [0; 4],
            image_size: [0; 4],
            x_per_m: [0; 4],
            y_per_m: [0; 4],
            colours_used: [0; 4],
            important_colours: [0; 4]
        };

        file.seek(SeekFrom::Start(14))?;

        file.read_exact(&mut info_header.size)?;
        file.read_exact(&mut info_header.width)?;
        file.read_exact(&mut info_header.height)?;
        file.read_exact(&mut info_header.planes)?;
        file.read_exact(&mut info_header.bits_per_px)?;
        file.read_exact(&mut info_header.compression)?;
        file.read_exact(&mut info_header.image_size)?;
        file.read_exact(&mut info_header.x_per_m)?;
        file.read_exact(&mut info_header.y_per_m)?;
        file.read_exact(&mut info_header.colours_used)?;
        file.read_exact(&mut info_header.important_colours)?;

        Ok(info_header)
    }
}

impl Clone for BmpInfoHeader {
    fn clone(&self) -> Self {
        BmpInfoHeader {
            size: self.size,
            width: self.width,
            height: self.height,
            planes: self.planes,
            bits_per_px: self.bits_per_px,
            compression: self.compression,
            image_size: self.image_size,
            x_per_m: self.x_per_m,
            y_per_m: self.y_per_m,
            colours_used: self.colours_used,
            important_colours: self.important_colours
        }
    }
}