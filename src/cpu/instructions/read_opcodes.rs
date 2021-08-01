use super::utils::*;
use crate::cpu::*;

pub fn instr_lda(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.reg_a = value;
    set_nz_flags(cpu, value);
}

pub fn instr_ldx(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.reg_x = value;
    set_nz_flags(cpu, value);
}

pub fn instr_ldy(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.reg_y = value;
    set_nz_flags(cpu, value);
}

pub fn instr_cmp(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::C, value <= cpu.reg_a);
    set_nz_flags(cpu, cpu.reg_a.wrapping_sub(value));
}

pub fn instr_and(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.reg_a &= value;
    set_nz_flags(cpu, cpu.reg_a);
}

pub fn instr_eor(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.reg_a ^= value;
    set_nz_flags(cpu, cpu.reg_a);
}

pub fn instr_ora(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.reg_a |= value;
    set_nz_flags(cpu, cpu.reg_a);
}

pub fn instr_bit(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::V, value & (1 << 6) != 0);
    cpu.status.set(CpuFlags::N, value & (1 << 7) != 0);
    cpu.status.set(CpuFlags::Z, value & cpu.reg_a == 0)
}

fn _add(cpu: &mut Cpu, value: u8) {
    let sum = cpu.reg_a as u16 + value as u16 + cpu.status.get_bit(CpuFlags::C) as u16;
    let result = sum as u8;
    cpu.status.set(CpuFlags::C, sum >> 8 != 0);
    cpu.status.set(
        CpuFlags::V,
        (cpu.reg_a ^ result) & (value & result) & 0x80 != 0,
    );
    cpu.reg_a = result;
    set_nz_flags(cpu, result);
}

pub fn instr_adc(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    _add(cpu, value);
}

pub fn instr_sbc(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    _add(cpu, ((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
}

pub fn instr_cpx(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::C, value <= cpu.reg_x);
    set_nz_flags(cpu, cpu.reg_x.wrapping_sub(value));
}

pub fn instr_cpy(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::C, value <= cpu.reg_y);
    set_nz_flags(cpu, cpu.reg_y.wrapping_sub(value));
}
