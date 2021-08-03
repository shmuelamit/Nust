use crate::cpu::mappers::Mapper;

pub struct Bus<'a> {
    pub ram: [u8; 0x800],
    pub prg_rom: &'a [u8],
    pub prg_ram: &'a [u8],
    pub chr_rom: &'a [u8],
    pub mapper: Box<dyn Mapper>,
    pub cycles: usize,
}

impl Bus<'_> {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize],
            0x2000..=0x401F => {
                println!("Warning: Unimplemented PPU register read mapping - current instruction did nothing"); 0
                // Warn so I can test TODO implement PPU register mappings
            },
            0x4020..=0x5FFF => todo!("Expansion ROM"),
            0x6000..=0x7FFF => todo!("SRAM and saving mechanisms"),
            0x8000..=0xFFFF => {
                self.prg_rom[self.mapper.cpu_map_read(addr).unwrap_or_default() as usize]
            }
            _ => {
                unreachable!("Bus reading match failed - impossible!")
            }
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read(addr) as u16) + ((self.read(addr + 1) as u16) << 8)
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram[(addr & 0x07FF) as usize] = value;
            }
            0x2000..=0x3FFF => {
                println!("Warning: Unimplemented PPU register write mapping - current instruction did nothing")
                // Warn so I can test TODO implement PPU register mappings
            }
            _ => {
                self.mapper.cpu_map_write(addr, value);
            }
        }
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write(addr, (value >> 8) as u8);
        self.write(addr + 1, (value & 0xFF) as u8);
    }

    pub fn cycle(&mut self, cycles: u8) {
        self.cycles += cycles as usize
    }
}
