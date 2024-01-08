use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn main() -> io::Result<()> {

    let mut file = File::open("sample2.bmp")?;
    let mut bmp = Bmp::build_from_file(&mut file)?;

    bmp.rotate_270();
    bmp.print_all(true, true);

    bmp.write_to_file("test")?;

    Ok(())
}
