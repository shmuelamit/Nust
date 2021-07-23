use super::utils::*;
use crate::cpu::*;

pub fn instr_lsr(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    let result = get_value(cpu, mode, input);
    if mode.is_input_address() {
        cpu.bus.write(input, result >> 1)
    } else {
        cpu.reg_a = result >> 1;
    }

    cpu.status.set(CpuFlags::C, result & 1 != 0);
    set_nz_flags(cpu, result);

    if mode.is_input_address() {
        cpu.bus.cycle(2)
    }
}

pub fn instr_bit(cpu: &mut Cpu, mode: AddresingMode, input: u16) {
    let input = cpu.bus.read_byte(input);
    cpu.status.set(CpuFlags::V, input & (1 << 6) != 0);
    cpu.status.set(CpuFlags::N, input & (1 << 7) != 0);
    cpu.status.set(CpuFlags::Z, input & cpu.reg_a == 0)
}
