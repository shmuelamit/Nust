use std::fs::File;
use std::io::Read;





mod cpu;
mod nes_parser;

fn main() {
    let mut f = File::open("Super_mario_brothers.nes").unwrap();
    let mut buffer = vec!();

    f.read_to_end(&mut buffer).unwrap();
    println!();
}
