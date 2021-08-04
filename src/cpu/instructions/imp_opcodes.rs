use crate::cpu::*;

use super::utils::*;

// "Implied instructions don't care about memory, they have alzheimer's."
//      - Alan turing

pub fn instr_sei(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.status.insert(CpuFlags::I)
}

pub fn instr_sed(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.status.insert(CpuFlags::D)
}

pub fn instr_sec(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.status.insert(CpuFlags::C)
}

pub fn instr_clc(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.status.remove(CpuFlags::C)
}

pub fn instr_cld(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.status.remove(CpuFlags::D)
}

pub fn instr_cli(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.status.remove(CpuFlags::I)
}

pub fn instr_clv(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.status.remove(CpuFlags::V)
}

pub fn instr_nop(_cpu: &mut Cpu, _mode: AddresingMode) {}

// incremnets and decrements

pub fn instr_inx(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_x = cpu.reg_x.wrapping_add(1);
    set_nz_flags(cpu, cpu.reg_x);
}

pub fn instr_dex(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_x = cpu.reg_x.wrapping_sub(1);
    set_nz_flags(cpu, cpu.reg_x);
}

pub fn instr_iny(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_y = cpu.reg_y.wrapping_add(1);
    set_nz_flags(cpu, cpu.reg_y);
}

pub fn instr_dey(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_y = cpu.reg_y.wrapping_sub(1);
    set_nz_flags(cpu, cpu.reg_y);
}

// transfers

pub fn instr_tax(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_x = cpu.reg_a;
    set_nz_flags(cpu, cpu.reg_x);
}

pub fn instr_tay(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_y = cpu.reg_a;
    set_nz_flags(cpu, cpu.reg_y);
}

pub fn instr_tsx(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_x = cpu.stack_pointer;
    set_nz_flags(cpu, cpu.reg_x);
}

pub fn instr_txs(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.stack_pointer = cpu.reg_x;
}

pub fn instr_txa(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_a = cpu.reg_x;
    set_nz_flags(cpu, cpu.reg_x);
}

pub fn instr_tya(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_a = cpu.reg_y;
    set_nz_flags(cpu, cpu.reg_a);
}
