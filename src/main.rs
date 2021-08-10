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
mod ppu;

fn dump_instructions(cpu: &mut Cpu, instr_count: u64) {}

fn main() {}
