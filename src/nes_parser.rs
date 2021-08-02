use bitflags::bitflags;
use nom::bytes::complete::tag;

use nom::error::context;
use nom::IResult;


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
    pub struct InesFlags6: u8 {
        const MIRRORING =   1u8 << 1;
        const PERSISTENCE = 1u8 << 2;
        const TRAINER =     1u8 << 3;
        const FOUR_SCREEN = 1u8 << 4;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct InesFlags7: u8 {
        const UNISYSTEM =   1u8 << 1;
        const PLAYCHOICE =  1u8 << 2;
        const NES2 =        1u8 << 4;
    }
}

pub struct InesHeaderFlags {
    flags6: InesFlags6,
    flags7: InesFlags7,
}

pub struct InesHeader {
    signature: [u8; 4],
    // NES<EOF>
    prg_size: u8,
    // In 16Kib units
    chr_size: u8,
    // In 8Kib units
    flags: InesHeaderFlags,
    mapper: u16,

    // rarely used
    prg_ram_size: Option<u8>,
    tv_system_type: Option<u8>,
}

pub fn sign_parse(input: &[u8]) -> IResult<&[u8], &[u8]> {
    context("Signature", tag(b"NES\x1A"))(input)
}