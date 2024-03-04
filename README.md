Work-in-progress Rust library for reading and manipulating bitmap (.bmp) files. Documentation [here](https://docs.rs/bumpy).



## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
bumpy = "0.0.2"
```

## Features

Currently supports reading 24-bit and 8-bit .bmp files into a mutable struct, performing manipulations on the image such as greyscaling, rotation, or mirroring the image, and then writing the struct to a bitmap file. Also support generating a 24 bit bitmap file of given width and height.

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
