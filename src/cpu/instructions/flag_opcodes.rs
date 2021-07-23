use super::utils::*;
use crate::cpu::*;

pub fn instr_sei(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    cpu.status.insert(CpuFlags::I)
}

pub fn instr_sed(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    cpu.status.insert(CpuFlags::D)
}

pub fn instr_sec(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    cpu.status.insert(CpuFlags::C)
}

pub fn instr_clc(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    cpu.status.remove(CpuFlags::C)
}

pub fn instr_cld(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    cpu.status.remove(CpuFlags::D)
}

pub fn instr_cli(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    cpu.status.remove(CpuFlags::I)
}

pub fn instr_clv(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    cpu.status.remove(CpuFlags::V)
}
