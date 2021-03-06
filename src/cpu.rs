use std::fmt;

use bitflags::bitflags;

use instructions::*;

use crate::bus::Bus;

pub mod instructions;

const STACK_START_ADDR: u16 = 0x100;

bitflags! {
    #[derive(Default)]
    pub struct CpuFlags: u8 {
        const C = 0b00000001;
        const Z = 0b00000010;
        const I = 0b00000100;
        const D = 0b00001000;
        const B = 0b00010000;
        const BS = 0b00100000; // The bullshit flag
        const V = 0b01000000;
        const N = 0b10000000;
    }
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

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(PC: {:#06x}\tA: {:x}\tX: {:x}\tY: {:x}\tP: {:#010b} ({:#02x})\tSP: {:#02x})",
            self.program_counter,
            self.reg_a,
            self.reg_x,
            self.reg_y,
            self.status.bits(),
            self.status.bits(),
            self.stack_pointer
        )
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

impl Cpu {
    pub fn execute_next(&mut self) {
        let opcode = self.opcode_table[self.bus.cpu_read(self.program_counter) as usize];

        (opcode.instr.execute)(self, opcode.addresing_mode);

        self.bus.cycle(opcode.cycle_count);
        self.program_counter += opcode.get_length()
    }

    pub fn stack_push(&mut self, value: u8) {
        self.bus
            .cpu_write(STACK_START_ADDR + self.stack_pointer as u16, value);
        self.stack_pointer -= 1;
    }

    pub fn stack_push_word(&mut self, value: u16) {
        self.stack_push((value >> 8) as u8);
        self.stack_push((value & 0xff) as u8);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.bus
            .cpu_read(STACK_START_ADDR + self.stack_pointer as u16)
    }

    pub fn stack_pop_word(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        hi << 8 | lo
    }

    pub fn get_opcode_table(&self) -> [Opcode; 256] {
        self.opcode_table
    }

    pub fn create_from_bus(bus: Bus) -> Self {
        Self {
            program_counter: bus.cpu_read_word(0xFFFC),
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: CpuFlags::BS,
            stack_pointer: 0xFD,
            opcode_table: instructions::get_opcode_table(),
            bus,
        }
    }
}
