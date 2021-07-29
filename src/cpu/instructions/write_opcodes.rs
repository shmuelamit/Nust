use super::utils::*;
use crate::cpu::*;

/*
Write instructions (STA, STX, STY)
According to 6502_cpu.txt the ways to handle the addressing modes are the following:
*/

pub fn instr_sta(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.bus.write(input, cpu.reg_a);
}

pub fn instr_sty(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.bus.write(input, cpu.reg_x);
}

pub fn instr_stx(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, cross) = read_instr_value(cpu, mode);
    cpu.bus.write(input, cpu.reg_y);
}