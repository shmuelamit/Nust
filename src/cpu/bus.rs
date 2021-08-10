use crate::cpu::mappers::{get_mapper, Mapper};
use crate::nes_parser::{Cartridge, InesFile};

pub struct Bus {
    ram: [u8; 0x800],
    crt: Cartridge,
    cycles: usize,
}

impl Bus {
    pub fn cpu_read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize],
            0x2000..=0x401F => {
                println!("Warning: Unimplemented PPU register read mapping - current instruction did nothing");
                0
                // Warn so I can test TODO implement PPU register mappings
            }
            0x4020..=0x5FFF => todo!("Expansion ROM"),
            0x6000..=0x7FFF => todo!("SRAM and saving mechanisms"),
            0x8000..=0xFFFF => {
                self.crt.prg_rom[self.crt.mapper.cpu_map_read(addr).unwrap_or_default() as usize]
            }
            _ => {
                unreachable!("Bus reading match failed - impossible!")
            }
        }
    }

    pub fn cpu_read_word(&self, addr: u16) -> u16 {
        (self.cpu_read(addr) as u16) | ((self.cpu_read(addr + 1) as u16) << 8)
    }

    pub fn cpu_read_zp_word(&self, addr: u8) -> u16 {
        (self.cpu_read(addr as u16) as u16)
            | ((self.cpu_read(addr.wrapping_add(1) as u16) as u16) << 8)
    }

    pub fn cpu_write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                println!("RAMWRT: {:02X} -> {:04X}", value, addr & 0x7FF);
                self.ram[(addr & 0x07FF) as usize] = value;
            }
            0x2000..=0x3FFF => {
                println!("Warning: Unimplemented PPU register write mapping - current instruction did nothing")
                // Warn so I can test TODO implement PPU register mappings
            }
            _ => {
                self.crt.mapper.cpu_map_write(addr, value);
            }
        }
    }

    pub fn cpu_write_word(&mut self, addr: u16, value: u16) {
        self.cpu_write(addr + 1, (value >> 8) as u8);
        self.cpu_write(addr, (value & 0xFF) as u8);
    }

    pub fn cpu_write_zp_word(&mut self, addr: u8, value: u16) {
        self.cpu_write(addr.wrapping_add(1) as u16, (value >> 8) as u8);
        self.cpu_write(addr as u16, (value & 0xFF) as u8);
    }

    pub fn cycle(&mut self, cycles: u8) {
        self.cycles += cycles as usize
    }

    pub fn get_cycles(&self) -> usize {
        self.cycles
    }

    pub fn create_from_crt(crt: Cartridge) -> Self {
        Bus {
            ram: [0; 0x800],
            crt,
            cycles: 7,
        }
    }
}
