use super::utils::*;
use crate::cpu::*;

/*
Read-Modify-Write instructions (ASL, LSR, ROL, ROR, INC, DEC)
According to 6502_cpu.txt the ways to handle the addressing modes are the following:
*/

fn general_shift(
    cpu: &mut Cpu,
    mode: AddresingMode,
    input: u16,
    value: u8,
    oper: fn(u8, u32) -> u8,
) {
    let newval = oper(value, 1);

    if mode.is_input_address() {
        cpu.bus.write(input, newval);
    } else {
        cpu.reg_a = newval;
    }

    set_nz_flags(cpu, newval);
}

pub fn instr_asl(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, _cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::C, value & (1u8 << 7) != 0);
    general_shift(cpu, mode, input, value, u8::wrapping_shl);
}

pub fn instr_lsr(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, _cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::C, value & 1 != 0);
    general_shift(cpu, mode, input, value, u8::wrapping_shr);
}

pub fn instr_rol(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, _cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::C, value & (1u8 << 7) != 0);
    general_shift(cpu, mode, input, value, u8::rotate_right);
}

pub fn instr_ror(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, value, _cross) = read_instr_value(cpu, mode);
    cpu.status.set(CpuFlags::C, value & 1 != 0);
    general_shift(cpu, mode, input, value, u8::rotate_right);
}

pub fn instr_inc(cpu: &mut Cpu, mode: AddresingMode) {
    let (_input, value, _cross) = read_instr_value(cpu, mode);
    let newval = value.wrapping_add(1);
    set_nz_flags(cpu, newval);
}

pub fn instr_dec(cpu: &mut Cpu, mode: AddresingMode) {
    let (_input, value, _cross) = read_instr_value(cpu, mode);
    let newval = value.wrapping_sub(1);
    set_nz_flags(cpu, newval);
}
