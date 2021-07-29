mod bus;
pub mod instructions;

use bitflags::bitflags;
use bus::Bus;
use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct CpuFlags: u8 {
        const C = 0b00000001;
        const Z = 0b00000010;
        const I = 0b00000100;
        const D = 0b00001000;
        const B = 0b00010000;
        const V = 0b01000000;
        const N = 0b10000000;
    }
}

#[derive(Clone, Copy)]
pub enum AddresingMode {
    NON, // Invalid Instruction
    ZPG, // Zero page
    ZPX, // Zero page, X
    ZPY, // Zero page, Y
    ABS, // Absolute
    ABX, // Absolute, X
    ABY, // Absolute, Y
    IND, // Indirect
    IMP, // Implied
    ACC, // Accumulator
    IMM, // Immediate
    REL, // Relative
    IDX, // (Indirect, X)
    IDY, // (Indirect), Y
}

#[derive(Clone, Copy)]
pub struct Instruction {
    pub name: &'static str,
    pub execute: fn(&mut Cpu, AddresingMode),
}

#[derive(Clone, Copy)]
pub struct Opcode {
    pub instr: Instruction,
    pub addresing_mode: AddresingMode,
    pub cycle_count: u8,
}

pub struct Cpu {
    pub program_counter: u16,
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub status: CpuFlags,
    pub stack_pointer: u8,
    opcode_table: [Opcode; 256],
    bus: Bus,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            program_counter: 0x6969,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: CpuFlags::empty(),
            stack_pointer: 0xFF,
            opcode_table: [Opcode::default(); 256],
            bus: Bus::default(),
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(PC: {:#06x}\tA: {:x}\tX: {:x}\tY: {:x}\tP: {:#010b}\tSP: {:#06x})",
            self.program_counter,
            self.reg_a,
            self.reg_x,
            self.reg_y,
            self.status.bits(),
            self.stack_pointer
        )
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            name: "INV",
            execute: |cpu: &mut Cpu, _mode: AddresingMode| {
                panic!(
                    "Invalid CPU instruction!\nCPU state at invalid instruction:\n{}",
                    cpu
                )
            },
        }
    }
}

impl Default for Opcode {
    fn default() -> Self {
        Self {
            instr: Instruction::default(),
            addresing_mode: AddresingMode::NON,
            cycle_count: 0,
        }
    }
}

impl CpuFlags {
    pub fn get_bit(&self, flag: Self) -> u8 {
        if self.contains(flag) {
            1
        } else {
            0
        }
    }
}

impl Opcode {
    fn get_length(&self) -> u16 {
        match self.addresing_mode {
            AddresingMode::IMP | AddresingMode::ACC => 1,
            AddresingMode::ZPG
            | AddresingMode::ZPX
            | AddresingMode::ZPY
            | AddresingMode::IMM
            | AddresingMode::REL
            | AddresingMode::IDX
            | AddresingMode::IDY => 2,
            _ => 3,
        }
    }
}

impl AddresingMode {
    fn is_input_address(&self) -> bool {
        match self {
            AddresingMode::NON | AddresingMode::IMP | AddresingMode::ACC | AddresingMode::IMM => {
                false
            }

            _ => true,
        }
    }
}

impl Cpu {
    pub fn execute_opcode(&mut self, opcode: Opcode) {
        self.program_counter += opcode.get_length();

        (opcode.instr.execute)(self, opcode.addresing_mode);

        self.bus.cycle(opcode.cycle_count)
    }
}
