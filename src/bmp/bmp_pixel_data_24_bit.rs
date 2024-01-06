use std::fs::File;
use std::io::{self, Write, Read, Seek, SeekFrom};

/// Builds a `BmpPixelData24Bit` struct from a file and the corresponding `BmpInfoHeader`.
///     
/// # Arguments
/// 
/// * `file` - A mutable reference to a `File` object.
/// * `info_header` - A reference to the corresponding `BmpInfoHeader`.
/// 
/// # Returns
/// 
/// Returns a `Result` containing the `BmpPixelData24Bit` if successful, or an `io::Error` if an error occurred.
/// Only supporting 24-bit pixel data for now

pub struct BmpPixelData24Bit {
    pub data: Vec<u8>,
}

impl BmpPixelData24Bit {

    pub fn new(width: u32, height: u32) -> Self {
        let padding_length = round_up_to_multiple_of_four(width * 3) - (width * 3);

        println!("Padding length: {}", padding_length);

        let mut data = Vec::new();

        for _i in 0..height {
            for _j in 0..width {
                //push 100 width times
                data.push(100);
                data.push(50);
                data.push(25);
            }

            //pushing 0 padding_length times
            for _j in 0..padding_length {
                data.push(0);
            }
        }

        BmpPixelData24Bit {
            data
        }

    }

    pub fn build_from_file(file: &mut File, data_offset: &[u8; 4]) -> io::Result<Self> {
        // Move the file cursor to the start of the pixel data
        let data_offset = u32::from_le_bytes(*data_offset) as u64;
        file.seek(SeekFrom::Start(data_offset))?;

        let mut pixel_data = BmpPixelData24Bit {
            data: Vec::new(),
        };

        file.read_to_end(&mut pixel_data.data)?;

        Ok(pixel_data)
    }

    pub fn write_to_file(&self, file: &mut File) -> io::Result<()> {
        file.write(&self.data)?;

        println!("Wrote BMP pixel data to file");

        Ok(())
    }
}

impl Clone for BmpPixelData24Bit {
    fn clone(&self) -> Self {
        BmpPixelData24Bit {
            data: self.data.clone(),
        }
    }
}

fn round_up_to_multiple_of_four(value: u32) -> u32 {
    ((value + 3) / 4) * 4
}