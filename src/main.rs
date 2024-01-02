use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn main() -> io::Result<()> {
    let mut file = File::open("sample2.bmp")?;
    let bmp = Bmp::build_from_file(&mut file)?;
    bmp.print_all();

    Ok(())
}
