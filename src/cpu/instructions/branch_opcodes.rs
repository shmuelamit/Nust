use super::utils::*;
use crate::cpu::*;

fn branch(cpu: &mut Cpu, mode: AddresingMode) {
    let (_input, value, _cross) = read_instr_value(cpu, mode);
    let value = value as i8;
    cpu.bus.cycle(1);

    let newpc = (cpu.program_counter as i16).wrapping_add(value as i16) as u16;
    if newpc >> 8 != cpu.program_counter >> 8 {
        cpu.bus.cycle(1);
    }

    cpu.program_counter = newpc;
}

pub fn instr_bcs(cpu: &mut Cpu, mode: AddresingMode) {
    if cpu.status.contains(CpuFlags::C) {
        branch(cpu, mode);
    }
}

pub fn instr_bcc(cpu: &mut Cpu, mode: AddresingMode) {
    if !cpu.status.contains(CpuFlags::C) {
        branch(cpu, mode);
    }
}

pub fn instr_beq(cpu: &mut Cpu, mode: AddresingMode) {
    if cpu.status.contains(CpuFlags::Z) {
        branch(cpu, mode);
    }
}

pub fn instr_bne(cpu: &mut Cpu, mode: AddresingMode) {
    if !cpu.status.contains(CpuFlags::Z) {
        branch(cpu, mode);
    }
}

pub fn instr_bmi(cpu: &mut Cpu, mode: AddresingMode) {
    if cpu.status.contains(CpuFlags::N) {
        branch(cpu, mode);
    }
}

pub fn instr_bpl(cpu: &mut Cpu, mode: AddresingMode) {
    if !cpu.status.contains(CpuFlags::N) {
        branch(cpu, mode);
    }
}

pub fn instr_bvs(cpu: &mut Cpu, mode: AddresingMode) {
    if cpu.status.contains(CpuFlags::V) {
        branch(cpu, mode);
    }
}

pub fn instr_bvc(cpu: &mut Cpu, mode: AddresingMode) {
    if !cpu.status.contains(CpuFlags::V) {
        branch(cpu, mode);
    }
}
