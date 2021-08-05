use crate::cpu::*;

pub fn get_value(cpu: &mut Cpu, mode: AddresingMode, input: u16) -> u8 {
    if mode.is_input_address() {
        cpu.bus.cpu_read(input)
    } else {
        input as u8
    }
}

pub fn set_nz_flags(cpu: &mut Cpu, result: u8) {
    cpu.status.set(CpuFlags::Z, result == 0);
    cpu.status.set(CpuFlags::N, result & (1 << 7) != 0);
}

// ASL, LSR, DEC, INC, ROL, ROR, STA don't need cross
fn does_current_instr_need_cross(cpu: &mut Cpu) -> bool {
    match cpu.get_opcode_table()[cpu.bus.cpu_read(cpu.program_counter) as usize].instr.name {
        "ASL" | "LSR" | "DEC" | "INC" | "ROL" | "ROR" | "STA" => false,
        _ => true
    }
}

// add and checks for page cross
fn add_chk_page_cross(cpu: &mut Cpu, addr: u16, offset: u16) -> (u16, bool) {
    let result = addr.wrapping_add(offset);
    (result, (addr >> 8 != result >> 8) && does_current_instr_need_cross(cpu))
}

// also returns if we crossed a page
pub fn get_input(cpu: &mut Cpu, addresing_mode: AddresingMode) -> (u16, bool) {
    let (argb, argw) = (
        cpu.bus.cpu_read(cpu.program_counter + 1),
        cpu.bus.cpu_read_word(cpu.program_counter + 1),
    );

    match addresing_mode {
        AddresingMode::NON => (0, false),
        AddresingMode::ZPG => (argb as u16, false),
        AddresingMode::ZPX => ((argb.wrapping_add(cpu.reg_x)) as u16, false),
        AddresingMode::ZPY => ((argb.wrapping_add(cpu.reg_y)) as u16, false),
        AddresingMode::ABS => (argw, false),
        AddresingMode::ABX => add_chk_page_cross(cpu, argw, cpu.reg_x as u16),
        AddresingMode::ABY => add_chk_page_cross(cpu, argw, cpu.reg_y as u16),
        AddresingMode::IND => (
            if argw & 0xFF == 0xFF { // Just IND having a stroke at page boundaries
                ((cpu.bus.cpu_read(argw & 0xFF00) as u16) << 8) as u16 | cpu.bus.cpu_read(argw) as u16
            } else {
                cpu.bus.cpu_read_word(argw)
            },
            false,
        ),
        AddresingMode::IMP => (0, false),
        AddresingMode::ACC => (cpu.reg_a as u16, false),
        AddresingMode::IMM => (argb as u16, false),
        AddresingMode::REL => (argb as u16, false),
        AddresingMode::IDX => (cpu.bus.cpu_read_zp_word(argb.wrapping_add(cpu.reg_x)), false),
        AddresingMode::IDY => add_chk_page_cross(cpu, cpu.bus.cpu_read_zp_word(argb), cpu.reg_y as u16),
    }
}

// SHOULD BE USED ONLY ONCED PER INSTRUCTION
// Does an eager read, hope that would not mess anything up
pub fn read_instr_value(cpu: &mut Cpu, mode: AddresingMode) -> (u16, u8, bool) {
    let (input, cross) = get_input(cpu, mode);
    let value = get_value(cpu, mode, input);
    if cross {
        cpu.bus.cycle(1)
    }
    (input, value, cross)
}
