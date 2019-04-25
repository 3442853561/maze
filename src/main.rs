extern crate png;

mod rand;

use rand::rand;

fn main(){
    let mut seed = 1;
    println!("{}", rand(&mut seed));
    println!("{}", rand(&mut seed));
    println!("Done!");
}
