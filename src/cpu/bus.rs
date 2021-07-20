pub struct Bus {
    pub tmp_memory: [u8; 0x10000],
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            tmp_memory: [0; 0x10000],
        }
    }
}

impl Bus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.tmp_memory[(addr as usize) % 0x10000]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) + ((self.read_byte(addr + 1) as u16) << 8)
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.tmp_memory[addr] = value
    }
}
