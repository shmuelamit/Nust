use super::utils::*;
use crate::cpu::*;

pub fn instr_pha(cpu: &mut Cpu, mode: AddresingMode) {
    cpu.stack_push(cpu.reg_a)
}

pub fn instr_php(cpu: &mut Cpu, mode: AddresingMode) {
    cpu.stack_push((cpu.status | CpuFlags::Bs | CpuFlags::B).bits());
    cpu.status.remove(CpuFlags::Bs | CpuFlags::B)
}

pub fn instr_pla(cpu: &mut Cpu, mode: AddresingMode) {
    cpu.reg_a = cpu.stack_pop();
    set_nz_flags(cpu, cpu.reg_a)
}

pub fn instr_plp(cpu: &mut Cpu, mode: AddresingMode) {
    // Panic shouldn't happen because we have all flag possibilities
    cpu.status = CpuFlags::from_bits(cpu.stack_pop()).unwrap();
    cpu.status.insert(CpuFlags::Bs)
}

pub fn instr_jmp(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, cross) = get_input(cpu, mode);
    cpu.program_counter = input;
}

pub fn instr_brk(cpu: &mut Cpu, mode: AddresingMode) {
    cpu.program_counter += 1;
    cpu.status.insert(CpuFlags::I);
    cpu.stack_push_word(cpu.program_counter);

    cpu.status.insert(CpuFlags::Bs);
    cpu.stack_push(cpu.status.bits());
    cpu.status.remove(CpuFlags::Bs);

    cpu.program_counter = cpu.bus.read_word(0xFFFE);
}

pub fn instr_rti(cpu: &mut Cpu, mode: AddresingMode) {
    // Panic shouldn't happen because we have all flag possibilities
    cpu.status = CpuFlags::from_bits(cpu.stack_pop()).unwrap();
    cpu.status.remove(CpuFlags::Bs | CpuFlags::B);

    cpu.program_counter = cpu.stack_pop_word();
}

pub fn instr_jsr(cpu: &mut Cpu, mode: AddresingMode) {
    let (input, cross) = get_input(cpu, mode);
    cpu.stack_push_word(cpu.program_counter.wrapping_sub(1));
    cpu.program_counter = input
}

pub fn instr_rts(cpu: &mut Cpu, mode: AddresingMode) {
    cpu.program_counter = cpu.stack_pop_word().wrapping_add(1);
}
