use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use crate::bmp::bmp_info_header::BmpInfoHeader;

/// A struct representing the BMP color table.
pub struct BmpColourTable {
    pub data: Vec<(u8, u8, u8, u8)>
}

impl BmpColourTable {

    /// Creates a new `BmpColorTable` struct.
    /// 
    /// # Returns
    /// 
    /// Returns a `BmpColorTable` struct.
    pub fn new() -> Self {
        BmpColourTable {
            data: Vec::new()
        }
    }

    /// Builds a `BmpColorTable` struct from a file and the corresponding `BmpInfoHeader`.
    ///
    /// # Arguments
    ///
    /// * `file` - A mutable reference to a `File` object.
    /// * `info_header` - A reference to the corresponding `BmpInfoHeader`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `BmpColorTable` if successful, or an `io::Error` if an error occurred.
    pub fn build_from_file(file: &mut File, info_header: &BmpInfoHeader) -> io::Result<Self> {
        let color_table_size = u32::from_le_bytes(info_header.colours_used) * 4;

        let mut color_table = BmpColourTable {
            data: vec![(0, 0, 0, 0); color_table_size as usize],
        };

        // Seek to the beginning of the color table at 0036h
        file.seek(SeekFrom::Start(54))?;

        let mut buffer = vec![0; (color_table_size) as usize];

        file.read(&mut buffer)?;

        //group every four entries in the color table into a tuple
        color_table.data = buffer.chunks_exact(4)
            .map(|chunk| (chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect::<Vec<_>>();

        Ok(color_table)
    }

    /// Writes the `BmpColorTable` to a file.
    ///     
    /// # Arguments
    ///     
    /// * `file` - A mutable reference to a `File` object.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing `()` if successful, or an `io::Error` if an error occurred.
    pub fn write_to_file(&self, file: &mut File) -> io::Result<()> {
        for (r, g, b, a) in &self.data {
            file.write(&[*b, *g, *r, *a])?;
        }

        println!("Wrote BMP color table to file");

        Ok(())
    }
}

impl Clone for BmpColourTable {
    fn clone(&self) -> Self {
        BmpColourTable {
            data: self.data.clone()
        }
    }
}