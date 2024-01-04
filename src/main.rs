use std::fs::File;
use std::io;
use bumpy::bmp::Bmp;

fn test() {
    let tuple_data = Vec::from([1, 2, 3, 4, 5, 6]);
    let mut new_tuple_data = Vec::new();
    let width = 3;
    let height = 2;

    for i in (1..=width).rev() {
        for j in 0..=(height - 1) {
            let index = ((i - 1) + height * j) as usize;
            println!("index: {}, value: {}", index, tuple_data[index]);
            new_tuple_data.push(tuple_data[index]);
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("sample.bmp")?;
    let bmp = Bmp::build_from_file(&mut file)?;

    let mut bmp2 = bmp.clone();

    bmp2.pixel_data.data[0] = 0;
    bmp2.pixel_data.data[1] = 255;
    bmp2.pixel_data.data[2] = 0;

    bmp2.print_all(false, false);

    bmp2.to_greyscale();
    bmp2.rotate_270();

    bmp2.write_to_file("test")?;

    test();

    Ok(())
}
