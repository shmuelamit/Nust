mod bus;
use bus::Bus;

use bitflags::bitflags;
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
    pub execute: fn(&mut Cpu, &Instruction, u16) -> u8,
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
    opcode_table: [Instruction; 256],
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
            opcode_table: [Instruction::default(); 256],
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
            addresing_mode: AddresingMode::NON,
            execute: |cpu: &mut Cpu, _instr: &Instruction, _input: u16| -> u8 {
                panic!(
                    "Invalid CPU instruction!\nCPU state at invalid instruction:\n{}",
                    cpu
                )
            },
            cycle_count: 69,
        }
    }
}

impl Instruction {
    fn is_input_address(&self) -> bool {
        match self.addresing_mode {
            AddresingMode::NON | AddresingMode::IMP | AddresingMode::ACC | AddresingMode::IMM => {
                false
            }

            _ => true,
        }
    }

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

impl Cpu {
    // Returns cycle count
    pub fn execute_instr(&mut self, instr: Instruction) -> u8 {
        let (argb, argw) = (
            self.bus.read_byte(self.program_counter + 1) as u16,
            self.bus.read_word(self.program_counter + 1) as u16,
        );

        self.program_counter += instr.get_length();

        instr.cycle_count
            + match instr.addresing_mode {
                AddresingMode::NON => (instr.execute)(self, &instr, 0),
                AddresingMode::ZPG => (instr.execute)(self, &instr, argb),
                AddresingMode::ZPX => (instr.execute)(self, &instr, argb + self.reg_x as u16),
                AddresingMode::ZPY => (instr.execute)(self, &instr, argb + self.reg_y as u16),
                AddresingMode::ABS => (instr.execute)(self, &instr, argw),
                AddresingMode::ABX => (instr.execute)(self, &instr, argw + self.reg_x as u16),
                AddresingMode::ABY => (instr.execute)(self, &instr, argw + self.reg_y as u16),
                AddresingMode::IND => (instr.execute)(self, &instr, self.bus.read_word(argw)),
                AddresingMode::IMP => (instr.execute)(self, &instr, 0),
                AddresingMode::ACC => (instr.execute)(self, &instr, self.reg_a as u16),
                AddresingMode::IMM => (instr.execute)(self, &instr, argb),
                AddresingMode::REL => (instr.execute)(self, &instr, argb),
                AddresingMode::IDX => {
                    (instr.execute)(self, &instr, self.bus.read_word(argw + self.reg_x as u16))
                }
                AddresingMode::IDY => {
                    (instr.execute)(self, &instr, self.bus.read_word(argw) + self.reg_y as u16)
                }
            }
    }

    pub fn execute_opcode(&mut self, opcode: u8) -> u8 {
        self.execute_instr(self.opcode_table[opcode as usize])
    }
}

fn main() {
    
}