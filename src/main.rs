use crate::cpu::instructions::addr_to_instr;
use crate::nes_parser::InesFile;
use nom::error::Error;
use nom::Err;
use std::fs;
use std::io::{stdin, Read};

mod cpu;
mod nes_parser;

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = fs::File::open(filename).expect("no file found");
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    println!(" - Testing stuff! - ");
    let ines_bytes = get_file_as_byte_vec("nestest.nes");
    let ines = match nes_parser::parse_ines_bytes(ines_bytes.as_slice()) {
        Ok(result) => result.1,
        Err(_) => panic!("Shit"),
    };
    println!("{:x?}", ines.header);
    let mut cpu = cpu::Cpu::create_from_ines(&ines);
    cpu.program_counter = 0xC000;
    loop {
        println!("Cpu status: {}", cpu);
        println!("{}", addr_to_instr(&cpu, cpu.program_counter));
        cpu.execute_next();
        stdin().read(&mut [0u8]).unwrap();
        println!();
    }
}
