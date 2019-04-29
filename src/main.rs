extern crate png;
use png::HasParameters;
use png::chunk::pHYs;
use png::{PixelDimensions, Unit};

extern crate failure;
use failure::Error;

mod rand;
pub use rand::rand;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

mod template;
pub use template::*;

fn pixel_dims_to_data(pixel_dims: PixelDimensions) -> Vec<u8> {
    let _xppu = pixel_dims.xppu.to_be_bytes();
    let _yppu = pixel_dims.yppu.to_be_bytes();
    vec![_xppu[0], _xppu[1], _xppu[2], _xppu[3], _yppu[0], _yppu[1], _yppu[2], _yppu[3], pixel_dims.unit as u8]
}

fn main() -> Result<(), Error>{
    for i in 0..3 {
        let mut seed = i;
        let path_str = format!("./image_{}.png", i);
        let path = Path::new(&path_str);
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 1500, 1500); 
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        
        let mut writer = encoder.write_header()?;
        let mut data = vec![255; 1500 * 1500 * 3];
        println!("{}: {}", i, rand(&mut seed));

        writer.write_chunk(pHYs, &pixel_dims_to_data(PixelDimensions{xppu: 11811u32, yppu: 11811u32, unit: Unit::Meter}))?;
        writer.write_image_data(&data)?;
    }
    println!("Done!");
    Ok(())
}

