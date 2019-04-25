extern crate png;
use png::HasParameters;

mod rand;
use rand::rand;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

fn main(){
    let mut seed = 1;
    
    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1500, 1500); 
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    
    let mut writer = encoder.write_header().unwrap();
    let mut data = vec![255; 1500 * 1500 * 3];
    
    // TODO
    
    writer.write_image_data(&data).unwrap();
    
    println!("{}", rand(&mut seed));
    println!("Done!");
}
