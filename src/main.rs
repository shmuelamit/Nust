use crate::cpu::instructions::addr_to_instr;
use crate::cpu::{Cpu, CpuFlags};
use crate::nes_parser::InesFile;
use nom::error::Error;
use nom::Err;
use std::fs;
use std::fs::OpenOptions;
use std::io::{stdin, Read, Write};

mod cpu;
mod nes_parser;

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = fs::File::open(filename).expect("no file found");
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn dump_current_instruction(cpu: &mut Cpu) -> String {
    let opcode = cpu.get_opcode_table()[cpu.bus.cpu_read(cpu.program_counter) as usize];
    let mut s = format!("{:04X} ", cpu.program_counter);
    for i in 0..=2 {
        if (i + 1) <= opcode.get_length() {
            s.push_str(&format!(" {:02X}", cpu.bus.cpu_read(cpu.program_counter + i)))
        } else {
            s.push_str(&"   ")
        }
    }
    s.push_str(&format!(
        " {:30} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:---,--- CYC:{}",
        addr_to_instr(&cpu, cpu.program_counter),
        cpu.reg_a,
        cpu.reg_x,
        cpu.reg_y,
        cpu.status.bits(),
        cpu.stack_pointer,
        cpu.bus.cycles
    ));
    s
}

fn dump_instructions(cpu: &mut Cpu, instr_count: u64) {}

fn main() {
    println!(" - Testing stuff! - ");
    let ines_bytes = get_file_as_byte_vec("nestest.nes");
    let ines = match nes_parser::parse_ines_bytes(ines_bytes.as_slice()) {
        Ok(result) => result.1,
        Err(_) => panic!("Shit"),
    };
    println!("{:x?}", ines.header);
    let mut cpu = Cpu::create_from_ines(&ines);
    cpu.program_counter = 0xc000;
    cpu.status = CpuFlags::from_bits_truncate(0x24);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("nustest.log")
        .expect("Unable to open file");
    for i in 1..8991 {
        if let Err(e) = writeln!(file, "{}", dump_current_instruction(&mut cpu)) {
            eprintln!("Couldn't write to file: {}", e);
        }
        println!(" - {} - ", i);
        cpu.execute_next();
        //stdin().read(&mut [0u8]).unwrap();
    }
}
