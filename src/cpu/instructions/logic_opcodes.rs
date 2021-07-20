fn set_logic_flags(cpu: &mut Cpu, instr: &Instruction) {
    cpu.status.all
}

pub fn instr_and(cpu: &mut Cpu, instr: &Instruction, input: u16) -> u8 {
    let m: u8 =  cpu.bus.read_byte(input) if instr.is_input_address() else input as u8;
    cpu.reg_a &= m;
}