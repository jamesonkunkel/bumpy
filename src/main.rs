use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn main() -> io::Result<()> {

    let mut bmp = Bmp::new(177, 69);

    bmp.rotate_90();
    bmp.write_to_file("test")?;

    let mut file = File::open("sample5.bmp")?;
    let mut bmp2 = Bmp::build_from_file(&mut file)?;

    
    bmp2.rotate_270();
    // bmp2.rotate_90_raw();
    bmp2.write_to_file("test2")?;

    Ok(())
}
