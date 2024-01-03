use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn main() -> io::Result<()> {
    let mut file = File::open("sample4.bmp")?;
    let bmp = Bmp::build_from_file(&mut file)?;

    let mut bmp2 = bmp.clone();

    bmp2.pixel_data.data[0] = 0;
    bmp2.pixel_data.data[1] = 255;
    bmp2.pixel_data.data[2] = 0;

    bmp2.print_all(false, false);

    bmp2.to_greyscale();    
    bmp2.write_to_file("test")?;

    Ok(())
}
