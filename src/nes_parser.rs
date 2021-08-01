use bitflags::bitflags;

// Decided to implement INES instead of NES 2.0 out of pure laziness
// might change later to the fancier format but for now we have backwards compatability

pub struct InesFile {
    header: InesHeader,
    trainer: Option<[u8; 512]>,
    pro_rom: Vec<u8>,
    chr_rom: Vec<u8>,

    // rarely used
    pc_inst_rom: Option<[u8; 8192]>,
    pc_prom: Option<[u8; 16]>,
}

bitflags! {
    #[derive(Default)]
    pub struct InesHeaderFlags: u32 {
        const mirroring =   1u32 << 1;
		const persistence = 1u32 << 2;
        const trainer =     1u32 << 3;
        const four_screen = 1u32 << 4;
        const unisystem =   1u32 << 5;
        const playchoice =  1u32 << 6;
        const tv_system =   1u32 << 7;
        const prg_ram =     1u32 << 8;
        const bus_confs =   1u32 << 8;
    }
}

pub struct InesHeader {
    signature: [u8; 4], // NES<EOF>
    prg_size: u8,       // In 16Kib units
    chr_size: u8,       // In 8Kib units
    flags: InesHeaderFlags,
    mapper: u16,

    // rarelu used
    prg_ram_size: Option<u8>,
    tv_system_type: Option<u8>
}
