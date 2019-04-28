extern crate png;
use png::HasParameters;

extern crate failure;
use failure::Error;

mod rand;
use rand::rand;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
 
fn main() -> Result<(), Error>{
    for i in 0..100 {
        let mut seed = i;
        let path_str = format!("./image_{}.png", i);
        let path = Path::new(&path_str);
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 1500, 1500); 
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        
        let mut writer = encoder.write_header()?;
        let mut data = vec![255; 1500 * 1500 * 3];
        
        
        
        writer.write_chunk([112, 72, 89, 115], &[0, 0, 46, 36, 0, 0, 46, 36, 1])?;
        writer.write_image_data(&data)?;
        println!("{}: {}", i, rand(&mut seed));
    }
    println!("Done!");
    Ok(())
}
