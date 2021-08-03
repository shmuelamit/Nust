use bitflags::bitflags;
use nom::{bytes::complete::{tag, take}, error::context, number::complete::be_u8, sequence::tuple, IResult, Err};
use std::fs::File;
use std::io::Read;
use nom::error::Error;

// Decided to implement INES instead of NES 2.0 out of pure laziness
// might change later to the fancier format but for now we have backwards compatability

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
    flags6: InesFlags6,
    flags7: InesFlags7,
}

#[derive(Debug)]
pub struct InesHeader {
    // NES<EOF>
    prg_size: u8,
    // In 16Kib units
    chr_size: u8,
    // In 8Kib units
    flags: InesHeaderFlags,
    mapper: u8,
}

#[derive(Debug)]
pub struct InesFile {
    header: InesHeader,
    trainer: Option<Vec<u8>>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
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
        tuple((sign_parse, be_u8, be_u8, mapper_flags_parse, take(8usize))),
    )(input)
    .map(|(next_input, res)| {
        let (_signature, prg_size, chr_size, (mapper, flags6, flags7), _) = res;
        (
            next_input,
            InesHeader {
                prg_size,
                chr_size,
                flags: InesHeaderFlags { flags6, flags7 },
                mapper,
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
        let (trainer, prg_rom, chr_rom) = (trainer.to_vec(), prg_rom.to_vec(), chr_rom.to_vec());
        (
            next_input,
            InesFile {
                header,
                trainer: if trainer.len() == 0 { None } else { Some(trainer) },
                prg_rom,
                chr_rom,
            },
        )
    })
}