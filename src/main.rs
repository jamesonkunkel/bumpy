use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn round_up_to_multiple_of_four(value: u32) -> u32 {
    ((value + 3) / 4) * 4
}

fn main() -> io::Result<()> {

    let mut bmp = Bmp::new(3, 2);
    println!("{:?}", bmp.pixel_data.data);
    bmp.rotate_90_raw();
    bmp.write_to_file("test")?;
    println!("{:?}", bmp.pixel_data.data);

    let mut file = File::open("sample6.bmp")?;
    let mut bmp2 = Bmp::build_from_file(&mut file)?;

    
    bmp2.rotate_180();
    bmp2.rotate_90_raw();
    bmp2.write_to_file("test2")?;

    Ok(())
}
