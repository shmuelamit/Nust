use crate::cpu::*;

pub fn get_value(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    if instr.is_input_address() {
        cpu.bus.read_byte(input)
    } else {
        input as u8
    }
}
