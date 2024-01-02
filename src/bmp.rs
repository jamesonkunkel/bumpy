mod bmp_header;
mod bmp_info_header;
mod bmp_colour_table;
mod bmp_pixel_data_8_bit;

//standard library imports
use std::fs::File;
use std::io;

//bmp file section imports
use bmp_header::BmpHeader;
use bmp_info_header::BmpInfoHeader;
use bmp_colour_table::BmpColourTable;
use bmp_pixel_data_8_bit::BmpPixelData8Bit;

/// Builds a `Bmp` struct from a file representing all of the contents of a .bmp file
///
/// # Arguments
///
/// * `file` - A mutable reference to a `File` object.
///
/// # Returns
///
/// Returns a `Result` containing the `BmpHeader` if successful, or an `io::Error` if an error occurred.
pub struct Bmp {
    pub header: BmpHeader,
    pub info_header: BmpInfoHeader,
    pub colour_table: BmpColourTable,
    pub pixel_data: BmpPixelData8Bit
}

impl Bmp {
    pub fn build_from_file(file: &mut File) -> io::Result<Self> {
        let header = BmpHeader::build_from_file(file)?;
        let info_header = BmpInfoHeader::build_from_file(file)?;
        let colour_table = BmpColourTable::build_from_file(file, &info_header)?;
        let pixel_data = BmpPixelData8Bit::build_from_file(file, &header.data_offset)?;

        Ok(Bmp {
            header,
            info_header,
            colour_table,
            pixel_data
        })
    }

    pub fn print_all(&self, with_color_table: bool) {
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

        if with_color_table {
            println!("BMP Color Table:");
            for (i, color) in self.colour_table.data.iter().enumerate() {
                println!("Color {}: {:?}", i, color);
            }
            println!("");
        }

        println!("BMP Pixel Data:");
        for (i, pixel) in self.pixel_data.data.iter().enumerate() {
            println!("Pixel {}: {}", i, pixel);
        }
    }

    pub fn write_to_file(&self, file_name: &str) -> io::Result<()> {
        let mut file = File::create(format!("{}.bmp", file_name))?;

        self.header.write_to_file(&mut file)?;
        self.info_header.write_to_file(&mut file)?;
        self.colour_table.write_to_file(&mut file)?;
        self.pixel_data.write_to_file(&mut file)?;

        Ok(())
    }

}

impl Clone for Bmp {
    fn clone(&self) -> Self {
        Bmp {
            header: self.header.clone(),
            info_header: self.info_header.clone(),
            colour_table: self.colour_table.clone(),
            pixel_data: self.pixel_data.clone()
        }
    }
}

