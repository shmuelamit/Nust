use crate::cpu::*;

pub fn get_value(cpu: &mut Cpu, mode: AddresingMode, input: u16) -> u8 {
    if mode.is_input_address() {
        cpu.bus.read_byte(input)
    } else {
        input as u8
    }
}

pub fn set_nz_flags(cpu: &mut Cpu, result: u8) {
    cpu.status.set(CpuFlags::Z, result == 0);
    cpu.status.set(CpuFlags::N, result & (1 << 7) != 0);
}

// add and checks for page cross
fn add_chk_page_cross(addr: u16, offset: u16) -> (u16, bool) {
    let result = addr.wrapping_add(offset);
    (result, addr >> 8 != result >> 8)
}

// also returns if we crossed a page
pub fn get_input(cpu: &mut Cpu, addresing_mode: AddresingMode) -> (u16, bool) {
    let (argb, argw) = (
        cpu.bus.read_byte(cpu.program_counter + 1),
        cpu.bus.read_word(cpu.program_counter + 1),
    );

    match addresing_mode {
        AddresingMode::NON => (0, false),
        AddresingMode::ZPG => (argb as u16, false),
        AddresingMode::ZPX => ((argb + cpu.reg_x) as u16, false),
        AddresingMode::ZPY => ((argb + cpu.reg_y) as u16, false),
        AddresingMode::ABS => (argw, false),
        AddresingMode::ABX => add_chk_page_cross(argw, cpu.reg_x as u16),
        AddresingMode::ABY => add_chk_page_cross(argw, cpu.reg_y as u16),
        AddresingMode::IND => (cpu.bus.read_word(argw), false),
        AddresingMode::IMP => (0, false),
        AddresingMode::ACC => (cpu.reg_a as u16, false),
        AddresingMode::IMM => (argb as u16, false),
        AddresingMode::REL => (argb as u16, false),
        AddresingMode::IDX => (cpu.bus.read_word(argw + cpu.reg_x as u16), false),
        AddresingMode::IDY => add_chk_page_cross(cpu.bus.read_word(argw), cpu.reg_y as u16),
    }
}
