use super::utils::*;
use crate::cpu::*;

/*
According to 6502_cpu.txt the ways to handle the addressing modes are the following:

  Absolute addressing

        #  address R/W description
       --- ------- --- ------------------------------------------
        1    PC     R  fetch opcode, increment PC
        2    PC     R  fetch low byte of address, increment PC
        3    PC     R  fetch high byte of address, increment PC
        4  address  R  read from effective address

  Zero page addressing

        #  address R/W description
       --- ------- --- ------------------------------------------
        1    PC     R  fetch opcode, increment PC
        2    PC     R  fetch address, increment PC
        3  address  R  read from effective address

  Zero page indexed addressing

        #   address  R/W description
       --- --------- --- ------------------------------------------
        1     PC      R  fetch opcode, increment PC
        2     PC      R  fetch address, increment PC
        3   address   R  read from address, add index register to it
        4  address+I* R  read from effective address

       Notes: I denotes either index register (X or Y).

              * The high byte of the effective address is always zero,
                i.e. page boundary crossings are not handled.

  Absolute indexed addressing

        #   address  R/W description
       --- --------- --- ------------------------------------------
        1     PC      R  fetch opcode, increment PC
        2     PC      R  fetch low byte of address, increment PC
        3     PC      R  fetch high byte of address,
                         add index register to low address byte,
                         increment PC
        4  address+I* R  read from effective address,
                         fix the high byte of effective address
        5+ address+I  R  re-read from effective address


  Indexed indirect addressing

        #    address   R/W description
       --- ----------- --- ------------------------------------------
        1      PC       R  fetch opcode, increment PC
        2      PC       R  fetch pointer address, increment PC
        3    pointer    R  read from the address, add X to it
        4   pointer+X   R  fetch effective address low
        5  pointer+X+1  R  fetch effective address high
        6    address    R  read from effective address

       Note: The effective address is always fetched from zero page,
             i.e. the zero page boundary crossing is not handled.

  Indirect indexed addressing

        #    address   R/W description
       --- ----------- --- ------------------------------------------
        1      PC       R  fetch opcode, increment PC
        2      PC       R  fetch pointer address, increment PC
        3    pointer    R  fetch effective address low
        4   pointer+1   R  fetch effective address high,
                           add Y to low byte of effective address
        5   address+Y*  R  read from effective address,
                           fix high byte of effective address
        6+  address+Y   R  read from effective address

       Notes: The effective address is always fetched from zero page,
              i.e. the zero page boundary crossing is not handled.

              * The high byte of the effective address may be invalid
                at this time, i.e. it may be smaller by $100.

              + This cycle will be executed only if the effective address
                was invalid during cycle #5, i.e. page boundary was crossed.
*/

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
