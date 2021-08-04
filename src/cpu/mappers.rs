use crate::nes_parser::InesFile;

mod mapper_0;
mod mapper_3;

pub trait Mapper {
    fn cpu_map_read(&self, addr: u16) -> Option<u16>;
    fn cpu_map_write(&mut self, addr: u16, value: u8) -> Option<u16>;
    fn ppu_map_read(&self, addr: u16) -> Option<u16>;
    fn ppu_map_write(&mut self, addr: u16, value: u8) -> Option<u16>;
}

pub fn get_mapper(ines: &InesFile) -> Option<Box<dyn Mapper>> {
    match ines.header.mapper {
        0 => Some(Box::new(mapper_0::Mapper0 {
            prg_banks: ines.header.prg_size,
            chr_banks: ines.header.chr_size,
        })),
        3 => Some(Box::new(mapper_3::Mapper3 {
            prg_banks: ines.header.prg_size,
            chr_banks: ines.header.chr_size,
            current_chrbank: 0,
        })),
        _ => None,
    }
}
