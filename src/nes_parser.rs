use bitflags::bitflags;
use nom::{
    bytes::complete::{tag, take},
    error::context,
    number::complete::be_u8,
    sequence::tuple,
    IResult,
};

// Decided to implement INES instead of NES 2.0 out of pure laziness
// might change later to the fancier format but for now we have backwards compatability
// also didn't implement some useless flags for cleanliness's sake, might implement them later

bitflags! {
    #[derive(Default)]
    pub struct InesFlags6: u8 {
        const MIRRORING =   1;
        const PERSISTENCE = 2;
        const TRAINER =     4;
        const FOUR_SCREEN = 8;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct InesFlags7: u8 {
        const UNISYSTEM =   1;
        const PLAYCHOICE =  2;
        const NES2 =        8;
    }
}

#[derive(Debug)]
pub struct InesHeaderFlags {
    pub flags6: InesFlags6,
    pub flags7: InesFlags7,
}

#[derive(Debug)]
pub struct InesHeader {
    // NES<EOF>
    pub prg_size: u8,
    // In 16Kib units
    pub chr_size: u8,
    // In 8Kib units
    pub flags: InesHeaderFlags,
    pub mapper: u8,
    pub prg_ram_size: u8,
}

#[derive(Debug)]
pub struct InesFile {
    pub header: InesHeader,
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
}

fn sign_parse(input: &[u8]) -> IResult<&[u8], &[u8]> {
    context("Signature", tag(b"NES\x1A"))(input)
}

fn mapper_flags_parse(input: &[u8]) -> IResult<&[u8], (u8, InesFlags6, InesFlags7)> {
    context("Flags6", tuple((be_u8, be_u8)))(input).map(|(next_input, res)| {
        let (b1, b2) = res;
        (
            next_input,
            (
                (b1 >> 4) | (b2 & 0b11110000u8),
                InesFlags6::from_bits_truncate(b1),
                InesFlags7::from_bits_truncate(b2),
            ),
        )
    })
}

fn parse_ines_header(input: &[u8]) -> IResult<&[u8], InesHeader> {
    context(
        "INES header parser",
        tuple((
            sign_parse,
            be_u8,
            be_u8,
            mapper_flags_parse,
            be_u8,
            take(7usize),
        )),
    )(input)
    .map(|(next_input, res)| {
        let (_signature, prg_size, chr_size, (mapper, flags6, flags7), prg_ram_size, _) = res;
        (
            next_input,
            InesHeader {
                prg_size,
                chr_size,
                flags: InesHeaderFlags { flags6, flags7 },
                mapper,
                prg_ram_size,
            },
        )
    })
}

pub fn parse_ines_bytes(input: &[u8]) -> IResult<&[u8], InesFile> {
    let (input, header) = parse_ines_header(input)?;
    context(
        "INES file parser",
        tuple((
            take(if header.flags.flags6.contains(InesFlags6::TRAINER) {
                256usize
            } else {
                0usize
            }),
            take(16384usize * header.prg_size as usize),
            take(8192usize * header.chr_size as usize),
        )),
    )(input)
    .map(|(next_input, res)| {
        let (trainer, prg_rom, chr_rom) = res;
        let prg_ram = vec![0; header.prg_ram_size as usize];
        let (trainer, prg_rom, chr_rom) = (trainer.to_vec(), prg_rom.to_vec(), chr_rom.to_vec());
        (
            next_input,
            InesFile {
                header,
                trainer: if trainer.len() == 0 {
                    None
                } else {
                    Some(trainer)
                },
                prg_rom,
                chr_rom,
                prg_ram,
            },
        )
    })
}
