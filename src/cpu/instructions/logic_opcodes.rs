use super::utils::*;
use crate::cpu::*;

fn set_logic_flags(cpu: &mut Cpu, result: u8) {
    cpu.status.set(CpuFlags::Z, result == 0);
    cpu.status.set(CpuFlags::N, result & (1 << 7) != 0);
}

pub fn instr_and(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.reg_a &= get_value(cpu, instr, input);
    set_logic_flags(cpu, cpu.reg_a);
    0
}

pub fn instr_eor(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.reg_a ^= get_value(cpu, instr, input);
    set_logic_flags(cpu, cpu.reg_a);
    0
}

pub fn instr_ora(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    cpu.reg_a ^= get_value(cpu, instr, input);
    set_logic_flags(cpu, cpu.reg_a);
    0
}

pub fn instr_lsr(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    let result = get_value(cpu, instr, input);
    if instr.is_input_address() {
        cpu.bus.write(input, result >> 1)
    } else {
        cpu.reg_a = result >> 1;
    }

    cpu.status.set(CpuFlags::C, result & 1 != 0);
    set_logic_flags(cpu, result);

    if instr.is_input_address() {
        2
    } else {
        0
    }
}

pub fn instr_bit(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    let input = cpu.bus.read_byte(input);
    cpu.status.set(CpuFlags::V, input & (1 << 6) != 0);
    cpu.status.set(CpuFlags::N, input & (1 << 7) != 0);
    cpu.status.set(CpuFlags::Z, input & cpu.reg_a == 0);
    0
}
