use crate::cpu::*;

use super::utils::*;

pub fn instr_pha(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.stack_push(cpu.reg_a)
}

pub fn instr_php(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.stack_push((cpu.status | CpuFlags::BS | CpuFlags::B).bits());
    cpu.status.remove(CpuFlags::BS | CpuFlags::B)
}

pub fn instr_pla(cpu: &mut Cpu, _mode: AddresingMode) {
    cpu.reg_a = cpu.stack_pop();
    set_nz_flags(cpu, cpu.reg_a)
}

pub fn instr_plp(cpu: &mut Cpu, mode: AddresingMode) {
    // Panic shouldn't happen because we have all flag possibilities
    cpu.status = CpuFlags::from_bits(cpu.stack_pop()).unwrap();
    cpu.status.insert(CpuFlags::BS)
}

pub fn instr_jmp(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, _cross) = get_input(cpu, mode);
    cpu.program_counter = input - mode.get_length();
}

pub fn instr_brk(cpu: &mut Cpu, mode: AddresingMode) {
    cpu.program_counter += 1;
    cpu.status.insert(CpuFlags::I);
    cpu.stack_push_word(cpu.program_counter);

    cpu.status.insert(CpuFlags::BS);
    cpu.stack_push(cpu.status.bits());
    cpu.status.remove(CpuFlags::BS);

    cpu.program_counter = cpu.bus.read_word(0xFFFE) - mode.get_length();
}

pub fn instr_rti(cpu: &mut Cpu, mode: AddresingMode) {
    // Panic shouldn't happen because we have all flag possibilities
    cpu.status = CpuFlags::from_bits(cpu.stack_pop()).unwrap();
    cpu.status.remove(CpuFlags::BS | CpuFlags::B);

    cpu.program_counter = cpu.stack_pop_word() - mode.get_length();
}

pub fn instr_jsr(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, _cross) = get_input(cpu, mode);
    cpu.stack_push_word(cpu.program_counter.wrapping_sub(1));
    cpu.program_counter = input - mode.get_length()
}

pub fn instr_rts(cpu: &mut Cpu, mode: AddresingMode) {
    cpu.program_counter = cpu.stack_pop_word().wrapping_add(1) - mode.get_length();
}
