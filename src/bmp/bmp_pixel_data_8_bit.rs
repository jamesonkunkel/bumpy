use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

/// Builds a `BmpPixelData8Bit` struct from a file and the corresponding `BmpInfoHeader`.
///     
/// # Arguments
/// 
/// * `file` - A mutable reference to a `File` object.
/// * `info_header` - A reference to the corresponding `BmpInfoHeader`.
/// 
/// # Returns
/// 
/// Returns a `Result` containing the `BmpPixelData8Bit` if successful, or an `io::Error` if an error occurred.
/// Only supporting 8-bit pixel data for now

pub struct BmpPixelData8Bit {
    pub data: Vec<u8>
}

impl BmpPixelData8Bit {
    pub fn build_from_file(file: &mut File, data_offset: &[u8; 4]) -> io::Result<Self> {
        // Move the file cursor to the start of the pixel data
        let data_offset = u32::from_le_bytes(*data_offset) as u64;
        file.seek(SeekFrom::Start(data_offset))?;

        let mut pixel_data = BmpPixelData8Bit {
            data: Vec::new()
        };

        file.read_to_end(&mut pixel_data.data)?;

        Ok(pixel_data)
    }
}

impl Clone for BmpPixelData8Bit {
    fn clone(&self) -> Self {
        BmpPixelData8Bit {
            data: self.data.clone()
        }
    }
}