use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn main() -> io::Result<()> {
    let mut file = File::open("sample5.bmp")?;
    let bmp = Bmp::build_from_file(&mut file)?;

    let mut bmp2 = bmp.clone();

    bmp2.info_header.x_per_m = u32::to_le_bytes(0);
    bmp2.info_header.y_per_m = u32::to_le_bytes(0);
    bmp2.rotate_180();

    bmp2.print_all_raw(false, false);

    bmp2.write_to_file("test")?;

    let bmp3 = Bmp::new(40, 100);

    bmp3.write_to_file("test2")?;

    bmp3.print_all(true, true);

    Ok(())
}
