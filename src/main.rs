use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn main() -> io::Result<()> {
    let mut file = File::open("sample8.bmp")?;
    let mut bmp = Bmp::build_from_file(&mut file)?;

    bmp.rotate_90();
    println!("{:?}", bmp.pixel_data.data);

    // bmp2.print_all(false, false);

    // bmp2.write_to_file("test")?;

    // let mut bmp3 = Bmp::new(3, 2);
    // // bmp3.rotate_90();
    // println!("{:?}", bmp3.pixel_data.data);

    bmp.write_to_file("test2")?;

    // bmp3.print_all(true, true);

    Ok(())
}
