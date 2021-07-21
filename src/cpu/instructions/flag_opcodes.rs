use super::utils::*;
use crate::cpu::*;

pub fn instr_sei(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.status.insert(CpuFlags::I); 0
}

pub fn instr_sed(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.status.insert(CpuFlags::D); 0
}

pub fn instr_sec(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.status.insert(CpuFlags::C); 0
}

pub fn instr_clc(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.status.remove(CpuFlags::C); 0
}

pub fn instr_cld(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.status.remove(CpuFlags::D); 0
}

pub fn instr_cli(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.status.remove(CpuFlags::I); 0
}

pub fn instr_clV(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.status.remove(CpuFlags::V); 0
}