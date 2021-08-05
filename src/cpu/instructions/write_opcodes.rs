use super::utils::*;
use crate::cpu::*;

/*
Write instructions (STA, STX, STY)
According to 6502_cpu.txt the ways to handle the addressing modes are the following:
*/

pub fn instr_sta(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, _value, _cross) = read_instr_value(cpu, mode);
    cpu.bus.cpu_write(input, cpu.reg_a);
}

pub fn instr_sty(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, _value, _cross) = read_instr_value(cpu, mode);
    cpu.bus.cpu_write(input, cpu.reg_y);
}

pub fn instr_stx(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, _value, _cross) = read_instr_value(cpu, mode);
    cpu.bus.cpu_write(input, cpu.reg_x);
}
