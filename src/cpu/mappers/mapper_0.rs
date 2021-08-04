use crate::cpu::mappers::Mapper;

pub(crate) struct Mapper0 {
    pub prg_banks: u8,
    pub chr_banks: u8,
}

impl Mapper for Mapper0 {
    fn cpu_map_read(&self, addr: u16) -> Option<u16> {
        if addr & 0x8000 != 0 {
            Some(addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF }))
        } else {
            None
        }
    }

    fn cpu_map_write(&mut self, addr: u16, value: u8) -> Option<u16> {
        if addr & 0x8000 != 0 {
            Some(addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF }))
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
