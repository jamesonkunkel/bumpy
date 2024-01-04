Work-in-progress Rust library for reading and manipulating bitmap (.bmp) files.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bumpy = "0.1"
```

## Example

```rust
extern crate bumpy;

use bumpy::bmp::Bmp;

//open a file
let mut file = File::open("sample.bmp")?;

//build a Bmp struct from the file
let bmp = Bmp::build_from_file(&mut file)?;

//do stuff like greyscale it
bmp.greyscale();

//write the modified bmp to a new file
bmp2.write_to_file("test")?;
```
