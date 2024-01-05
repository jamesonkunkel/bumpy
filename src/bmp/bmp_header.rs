use std::fs::File;
use std::io::{self, Read, Write};

#[repr(C)]
pub struct BmpHeader {
    pub signature: [u8; 2],
    pub file_size: [u8; 4],
    pub reserved: [u8; 4],
    pub data_offset: [u8; 4]
}

impl BmpHeader {

    pub fn new(width: u32, height: u32) -> Self {
        let file_size = u32::to_le_bytes((width * height * 3) + 54);

        BmpHeader {
            signature: [66, 77],
            file_size,
            reserved: [0; 4],
            data_offset: u32::to_le_bytes(54)
        }
    }

    pub fn build_from_file(file: &mut File) -> io::Result<Self> {
        let mut header = BmpHeader {
            signature: [0; 2],
            file_size: [0; 4],
            reserved: [0; 4],
            data_offset: [0; 4]
        };

        // Read the BMP file header
        file.read_exact(&mut header.signature)?;
        file.read_exact(&mut header.file_size)?;
        file.read_exact(&mut header.reserved)?;
        file.read_exact(&mut header.data_offset)?;

        Ok(header)
    }

    pub fn write_to_file(&self, file: &mut File) -> io::Result<()> {
        file.write(&self.signature)?;
        file.write(&self.file_size)?;
        file.write(&self.reserved)?;
        file.write(&self.data_offset)?;

        println!("Wrote BMP header to file");

        Ok(())
    }
}

impl Clone for BmpHeader {
    fn clone(&self) -> Self {
        BmpHeader {
            signature: self.signature,
            file_size: self.file_size,
            reserved: self.reserved,
            data_offset: self.data_offset
        }
    }
}