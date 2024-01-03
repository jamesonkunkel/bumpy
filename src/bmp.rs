mod bmp_header;
mod bmp_info_header;
mod bmp_colour_table;
mod bmp_pixel_data_24_bit;

//standard library imports
use std::fs::File;
use std::io;

//bmp file section imports
use bmp_header::BmpHeader;
use bmp_info_header::BmpInfoHeader;
use bmp_colour_table::BmpColourTable;
use bmp_pixel_data_24_bit::BmpPixelData24Bit;

/// A clonable struct representing a .bmp file. Top level abstraction of bitmap file. Currently only supports 24-bit .bmp files.
pub struct Bmp {
    pub header: BmpHeader,
    pub info_header: BmpInfoHeader,
    pub colour_table: BmpColourTable,
    pub pixel_data: BmpPixelData24Bit
}

impl Bmp {

    /// Builds a Bmp struct instance from a `File` object.
    ///
    /// # Arguments
    ///
    /// * `file` - A mutable reference to a `File` object.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `BmpHeader` if successful, or an `io::Error` if an error occurred.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::fs::File;
    /// use std::io;
    /// use bumpy::bmp::Bmp;
    ///     
    /// fn main() -> io::Result<()> {
    ///    let mut file = File::open("sample.bmp")?;
    ///    let bmp = Bmp::build_from_file(&mut file)?;
    /// 
    ///   Ok(())
    /// }
    /// ```   
    pub fn build_from_file(file: &mut File) -> io::Result<Self> {
        let header = BmpHeader::build_from_file(file)?;
        let info_header = BmpInfoHeader::build_from_file(file)?;
        let colour_table = BmpColourTable::build_from_file(file, &info_header)?;
        let pixel_data = BmpPixelData24Bit::build_from_file(file, &header.data_offset)?;

        if u16::from_le_bytes(info_header.bits_per_px) != 24 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Only 24-bit .bmp files are supported"));
        }

        Ok(Bmp {
            header,
            info_header,
            colour_table,
            pixel_data
        })
    }

    /// Prints the contents of the `Bmp` struct to the console.
    ///     
    /// # Arguments
    /// 
    /// * `with_color_table` - A boolean value indicating whether to print the color table.
    /// * `with_pixel_data` - A boolean value indicating whether to print the pixel data.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::fs::File;
    /// use std::io;
    /// use bumpy::bmp::Bmp;
    /// 
    /// fn main() -> io::Result<()> {
    ///   let mut file = File::open("sample.bmp")?;
    ///   let bmp = Bmp::build_from_file(&mut file)?;
    ///     
    ///   bmp.print_all(false, false);
    ///     
    ///   Ok(())
    /// }
    /// ```
    pub fn print_all(&self, with_color_table: bool, with_pixel_data: bool) {
        println!("BMP Header:");
        println!("Signature: {}", String::from_utf8_lossy(&self.header.signature));
        println!("File size: {}", u32::from_le_bytes(self.header.file_size));
        println!("Reserved: {}", u32::from_le_bytes(self.header.reserved));
        println!("Data offset: {}", u32::from_le_bytes(self.header.data_offset));
        println!("");

        println!("BMP Info Header:");
        println!("Size: {}", u32::from_le_bytes(self.info_header.size));
        println!("Width: {:?}", self.info_header.width);
        println!("Height: {:?}", self.info_header.height);
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

        if with_pixel_data {
            println!("BMP Pixel Data:");
        }
        
    }


    /// Writes the contents of the `Bmp` struct to a file.
    ///     
    /// # Arguments
    /// 
    /// * `file_name` - A string slice containing the name of the file to write to.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::fs::File;
    /// use std::io;
    /// use bumpy::bmp::Bmp;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut file = File::open("sample.bmp")?;
    ///     let bmp = Bmp::build_from_file(&mut file)?;
    ///     
    ///     bmp.write_to_file("test")?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn write_to_file(&self, file_name: &str) -> io::Result<()> {
        let mut file = File::create(format!("{}.bmp", file_name))?;

        self.header.write_to_file(&mut file)?;
        self.info_header.write_to_file(&mut file)?;
        self.colour_table.write_to_file(&mut file)?;
        self.pixel_data.write_to_file(&mut file)?;

        Ok(())
    }

    // internal function to convert pixel data to tuple data for 24-bit bmps
    fn to_tuple_data(&self) -> Vec<(u8, u8, u8)> {
        let mut tuple_data = Vec::new();

        for i in 0..self.pixel_data.data.len() / 3 {
            let b = self.pixel_data.data[i * 3];
            let g = self.pixel_data.data[i * 3 + 1];
            let r = self.pixel_data.data[i * 3 + 2];

            tuple_data.push((b, g, r));
        }

        tuple_data
    }

    /// Converts the pixel data to greyscale.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::fs::File;
    /// use std::io;
    /// use bumpy::bmp::Bmp;
    /// 
    /// fn main() -> io::Result<()> {
    ///     let mut file = File::open("sample.bmp")?;
    ///     let mut bmp = Bmp::build_from_file(&mut file)?;
    /// 
    ///     bmp.to_greyscale();
    /// 
    ///     bmp.write_to_file("test")?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn to_greyscale(&mut self) {
        let tuple_data = self.to_tuple_data();

        for (i, pixel) in tuple_data.iter().enumerate() {
            let (b, g, r) = rgb_to_greyscale(*pixel);

            self.pixel_data.data[i * 3] = b;
            self.pixel_data.data[i * 3 + 1] = g;
            self.pixel_data.data[i * 3 + 2] = r;
        }
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


//converts BGR tuple to grayscale
fn rgb_to_greyscale(bgr: (u8, u8, u8)) -> (u8, u8, u8) {
    let (b, g, r) = bgr;
    let grey_value = (0.299 * f64::from(r) + 0.587 * f64::from(g) + 0.114 * f64::from(b)).round() as u8;

    (grey_value, grey_value, grey_value)
}

