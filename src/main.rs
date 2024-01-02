use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn main() -> io::Result<()> {
    let mut file = File::open("sample2.bmp")?;
    let bmp = Bmp::build_from_file(&mut file)?;

    let mut bmp2 = bmp.clone();

    // bmp2.info_header.width = u32::to_le_bytes(2);
    // bmp2.info_header.height = u32::to_le_bytes(8);

    bmp2.pixel_data.data[10] = 40 as u8;

    bmp2.print_all(true);
    bmp2.write_to_file("test")?;

    Ok(())
}
