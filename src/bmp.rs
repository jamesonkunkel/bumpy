pub mod bmp_header;
pub mod bmp_info_header;
pub mod bmp_colour_table;
pub mod bmp_pixel_data;
mod utils;

//standard library imports
use std::fs::File;
use std::io;

//bmp file section imports
use bmp_header::BmpHeader;
use bmp_info_header::BmpInfoHeader;
use bmp_colour_table::BmpColourTable;
use bmp_pixel_data::BmpPixelData;

//import utils
use utils::{round_up_to_multiple_of_four, rgb_to_greyscale};

/// A clonable struct representing a .bmp file. Top level abstraction of bitmap file. Currently only supports 24-bit .bmp files.
pub struct Bmp {
    pub header: BmpHeader,
    pub info_header: BmpInfoHeader,
    pub colour_table: BmpColourTable,
    pub pixel_data: BmpPixelData
}

impl Bmp {

    /// Builds a `Bmp` struct instance.
    /// 
    /// # Arguments
    /// 
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bumpy::bmp::Bmp;
    /// 
    /// let bmp = Bmp::new(100, 100);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let header = BmpHeader::new(width, height);
        let info_header = BmpInfoHeader::new(width, height);
        let colour_table = BmpColourTable::new();
        let pixel_data = BmpPixelData::new(width, height);

        Bmp {
            header,
            info_header,
            colour_table,
            pixel_data
        }
    }

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
        let pixel_data = BmpPixelData::build_from_file(file, &header.data_offset)?;

        let bits_per_px = u16::from_le_bytes(info_header.bits_per_px);

        if bits_per_px != 24 && bits_per_px != 8{
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
        println!("Width: {:?}", u32::from_le_bytes(self.info_header.width));
        println!("Height: {:?}", u32::from_le_bytes(self.info_header.height));
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
            for (i, pixel) in self.pixel_data.data.iter().enumerate() {
                println!("Entry {}: {:?}", i, pixel);
            }
        }
        
    }

    /// Prints the contents of the `Bmp` struct to the console without converting the bytes to their corresponding integer values.
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
    ///     let mut file = File::open("sample.bmp")?;
    ///     let bmp = Bmp::build_from_file(&mut file)?;
    ///     
    ///     bmp.print_all_raw(false, false);
    ///     
    ///     Ok(())
    /// }
    /// ```

    pub fn print_all_raw (&self, with_color_table: bool, with_pixel_data: bool) {
        println!("BMP Header:");
        println!("Signature: {:?}", self.header.signature);
        println!("File size: {:?}", self.header.file_size);
        println!("Reserved: {:?}", self.header.reserved);
        println!("Data offset: {:?}", self.header.data_offset);
        println!("");

        println!("BMP Info Header:");
        println!("Size: {:?}", self.info_header.size);
        println!("Width: {:?}", self.info_header.width);
        println!("Height: {:?}", self.info_header.height);
        println!("Planes: {:?}", self.info_header.planes);
        println!("Bits per pixel: {:?}", self.info_header.bits_per_px);
        println!("Compression: {:?}", self.info_header.compression);
        println!("Image size: {:?}", self.info_header.image_size);
        println!("X pixels per meter: {:?}", self.info_header.x_per_m);
        println!("Y pixels per meter: {:?}", self.info_header.y_per_m);
        println!("Colours used: {:?}", self.info_header.colours_used);
        println!("Important colours: {:?}", self.info_header.important_colours);
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

    /// Converts the image to greyscale.
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
        let bits_per_px = u16::from_le_bytes(self.info_header.bits_per_px);

        if bits_per_px == 24 {
            self.to_greyscale_24();
        }
        else {
            panic!("Only 24-bit .bmp files are supported");
        }
    }

    // 24 bit function definition for greyscale conversion
    fn to_greyscale_24(&mut self){
        let width = u32::from_le_bytes(self.info_header.width);
        let height = u32::from_le_bytes(self.info_header.height);

        for i in (0..=width - 1).rev(){
            for j in 0..=height - 1 {
                let index = (i + width * j) as usize;
                let (b, g, r) = rgb_to_greyscale((self.pixel_data.data[index * 3], self.pixel_data.data[index * 3 + 1], self.pixel_data.data[index * 3 + 2]));

                self.pixel_data.data[index * 3] = b;
                self.pixel_data.data[index * 3 + 1] = g;
                self.pixel_data.data[index * 3 + 2] = r;
            }
        }
    }

    /// Rotates image 90 degrees clockwise.
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
    ///     bmp.rotate_90();
    /// 
    ///     bmp.write_to_file("test")?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn rotate_90(&mut self){
        let bits_per_px = u16::from_le_bytes(self.info_header.bits_per_px);

        if bits_per_px == 8 {
            self.rotate_90_8();
        }
        else if bits_per_px == 24 {
            self.rotate_90_24();
        }
        else {
            panic!("Only 24-bit .bmp files are supported");
        }
    }

    fn rotate_90_8(&mut self){
        let width = u32::from_le_bytes(self.info_header.width);
        let height = u32::from_le_bytes(self.info_header.height);

        let mut new_pixel_data = Vec::new();

        let curr_padding = round_up_to_multiple_of_four(width) - (width);
        let new_padding = round_up_to_multiple_of_four(height) - (height);

        for i in (0..=width - 1).rev() {
            for j in 0..=(height - 1) {
                let index = (i + width * j) as usize;
                new_pixel_data.push(self.pixel_data.data[index + (curr_padding * j) as usize]);
            }

            for _j in 0..new_padding {
                new_pixel_data.push(0);
            }
        }

        // swap width and height
        let width = self.info_header.width;
        let height = self.info_header.height;

        self.info_header.width = height;
        self.info_header.height = width;

        self.pixel_data.data = new_pixel_data;
    }

    //24 bit function definition for 90 degree rotation
    fn rotate_90_24(&mut self){
        let width = u32::from_le_bytes(self.info_header.width);
        let height = u32::from_le_bytes(self.info_header.height);

        let mut new_pixel_data = Vec::new();

        let curr_padding = round_up_to_multiple_of_four(width * 3) - (width * 3);
        let new_padding = round_up_to_multiple_of_four(height * 3) - (height * 3);

        for i in (0..=width - 1).rev() {
            for j in 0..=(height - 1) {
                let index = (i + width * j) as usize;
                new_pixel_data.push(self.pixel_data.data[index * 3 + (curr_padding * j) as usize]);
                new_pixel_data.push(self.pixel_data.data[index * 3 + 1 + (curr_padding * j) as usize]);
                new_pixel_data.push(self.pixel_data.data[index * 3 + 2 + (curr_padding * j) as usize]);
            }

            for _j in 0..new_padding {
                new_pixel_data.push(0);
            }
        }

        // swap width and height
        let width = self.info_header.width;
        let height = self.info_header.height;

        self.info_header.width = height;
        self.info_header.height = width;

        self.pixel_data.data = new_pixel_data;
    }
    
    /// Rotates image 180 degrees
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
    ///     bmp.rotate_180();
    /// 
    ///     bmp.write_to_file("test")?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn rotate_180(&mut self){
        let bits_per_px = u16::from_le_bytes(self.info_header.bits_per_px);
        
        if bits_per_px == 8 {
            self.rotate_90_8();
            self.rotate_90_8();
        }
        else if bits_per_px == 24 {
            self.rotate_90_24();
            self.rotate_90_24();
        }
        else {
            panic!("Only 24-bit .bmp files are supported");
        }
    }

    /// Rotates image 270 degrees clockwise.
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
    ///     bmp.rotate_270();
    /// 
    ///     bmp.write_to_file("test")?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn rotate_270(&mut self){
        let bits_per_px = u16::from_le_bytes(self.info_header.bits_per_px);

        if bits_per_px == 8 {
            self.rotate_90_8();
            self.rotate_90_8();
            self.rotate_90_8();
        }
        else if bits_per_px == 24 {
            self.rotate_90_24();
            self.rotate_90_24();
            self.rotate_90_24();
        }
        else {
            panic!("Only 24-bit .bmp files are supported");
        }
    }

    /// Mirrors image along horizontal axis
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
    ///     bmp.flip_hor();
    /// 
    ///     bmp.write_to_file("test")?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    pub fn flip_hor(&mut self){
        let width = u32::from_le_bytes(self.info_header.width);
        let height = u32::from_le_bytes(self.info_header.height);

        let mut new_pixel_data = Vec::new();

        let curr_padding = round_up_to_multiple_of_four(width * 3) - (width * 3);

        for i in 0..=height - 1 {
            for j in (1..=width).rev() {
                let index = ((j - 1) + width * i) as usize;
                new_pixel_data.push(self.pixel_data.data[index * 3 + (curr_padding * i) as usize]);
                new_pixel_data.push(self.pixel_data.data[index * 3 + 1 + (curr_padding * i) as usize]);
                new_pixel_data.push(self.pixel_data.data[index * 3 + 2 + (curr_padding * i) as usize]);
            }

            for _j in 0..curr_padding {
                new_pixel_data.push(0);
            }
        }

        self.pixel_data.data = new_pixel_data;


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

