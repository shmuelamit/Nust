use crate::bus::mappers::Mapper;

pub(crate) struct Mapper3 {
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub current_chrbank: u8,
}

impl Mapper for Mapper3 {
    fn cpu_map_read(&self, addr: u16) -> Option<u16> {
        if addr & 0x8000 != 0 {
            match self.prg_banks {
                1 => Some(addr & 0x3FFF),
                2 => Some(addr & 0x7FFF),
                _ => None,
            }
        } else {
            None
        }
    }

    fn cpu_map_write(&mut self, addr: u16, value: u8) -> Option<u16> {
        if addr & 0x8000 != 0 {
            self.current_chrbank = value & 3;
            Some(addr)
        } else {
            None
        }
    }

    fn ppu_map_read(&self, addr: u16) -> Option<u16> {
        todo!("PPU read")
    }

    fn ppu_map_write(&mut self, addr: u16, value: u8) -> Option<u16> {
        todo!("PPU write")
    }
}
