use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

/// Represents the header of a BMP file.
pub struct Bmp {
    pub header: BmpHeader,
    pub info_header: BmpInfoHeader,
    pub color_table: BmpColorTable,
    pub pixel_data: BmpPixelData8Bit
}

impl Bmp {
    pub fn build_from_file(file: &mut File) -> io::Result<Self> {
        let header = BmpHeader::build_from_file(file)?;
        let info_header = BmpInfoHeader::build_from_file(file)?;
        let color_table = BmpColorTable::build_from_file(file, &info_header)?;
        let pixel_data = BmpPixelData8Bit::build_from_file(file, &header.data_offset)?;

        Ok(Bmp {
            header,
            info_header,
            color_table,
            pixel_data
        })
    }

    pub fn print_all(&self) {
        println!("BMP Header:");
        println!("Signature: {}", String::from_utf8_lossy(&self.header.signature));
        println!("File size: {}", u32::from_le_bytes(self.header.file_size));
        println!("Reserved: {}", u32::from_le_bytes(self.header.reserved));
        println!("Data offset: {}", u32::from_le_bytes(self.header.data_offset));
        println!("");

        println!("BMP Info Header:");
        println!("Size: {}", u32::from_le_bytes(self.info_header.size));
        println!("Width: {}", u32::from_le_bytes(self.info_header.width));
        println!("Height: {}", u32::from_le_bytes(self.info_header.height));
        println!("Planes: {}", u16::from_le_bytes(self.info_header.planes));
        println!("Bits per pixel: {}", u16::from_le_bytes(self.info_header.bits_per_px));
        println!("Compression: {}", u32::from_le_bytes(self.info_header.compression));
        println!("Image size: {}", u32::from_le_bytes(self.info_header.image_size));
        println!("X pixels per meter: {}", u32::from_le_bytes(self.info_header.x_per_m));
        println!("Y pixels per meter: {}", u32::from_le_bytes(self.info_header.y_per_m));
        println!("Colours used: {}", u32::from_le_bytes(self.info_header.colours_used));
        println!("Important colours: {}", u32::from_le_bytes(self.info_header.important_colours));
        println!("");

        // println!("BMP Color Table:");
        // for (i, color) in self.color_table.data.iter().enumerate() {
        //     println!("Color {}: {:?}", i, color);
        // }

        println!("BMP Pixel Data:");
        for (i, pixel) in self.pixel_data.data.iter().enumerate() {
            println!("Pixel {}: {}", i, pixel);
        }
    }
}

/// Builds a `BmpHeader` struct from a file.
///
/// # Arguments
///
/// * `file` - A mutable reference to a `File` object.
///
/// # Returns
///
/// Returns a `Result` containing the `BmpHeader` if successful, or an `io::Error` if an error occurred.

#[repr(C)]
pub struct BmpHeader {
    signature: [u8; 2],
    file_size: [u8; 4],
    reserved: [u8; 4],
    data_offset: [u8; 4]
}

impl BmpHeader {
    fn build_from_file(file: &mut File) -> io::Result<Self> {
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
}

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
    size: [u8; 4],
    width: [u8; 4],
    height: [u8; 4],
    planes: [u8; 2],
    bits_per_px: [u8; 2],
    compression: [u8; 4],
    image_size: [u8; 4],
    x_per_m: [u8; 4],
    y_per_m: [u8; 4],
    colours_used: [u8; 4],
    important_colours: [u8; 4]
}

impl BmpInfoHeader {
    fn build_from_file(file: &mut File) -> io::Result<Self> {
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
pub struct BmpColorTable {
    data: Vec<(u8, u8, u8, u8)>
}

impl BmpColorTable {
    fn build_from_file(file: &mut File, info_header: &BmpInfoHeader) -> io::Result<Self> {
        let color_table_size = u32::from_le_bytes(info_header.colours_used) * 4;

        let mut color_table = BmpColorTable {
            data: vec![(0, 0, 0, 0); color_table_size as usize],
        };

        // Seek to the beginning of the color table at 0036h
        file.seek(SeekFrom::Start(54))?;

        let mut buffer = vec![0; (color_table_size * 4) as usize];

        file.read(&mut buffer)?;

        //group every four entries in the color table into a tuple
        color_table.data = buffer.chunks_exact(4)
            .map(|chunk| (chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect::<Vec<_>>();

        Ok(color_table)
    }
}

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
    data: Vec<u8>
}

impl BmpPixelData8Bit {
    fn build_from_file(file: &mut File, data_offset: &[u8; 4]) -> io::Result<Self> {
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