extern crate nom;
use nom::{IResult, named, verify};
use nom::number::complete::*;

#[derive(Debug)]
pub struct BhdHeader {
    pub magic: u32,
//    pub unk04: i8,
//    pub unk05: i8,
//    pub unk06: i8,
//    pub unk07: i8,
//    pub unk08: u32,
//    pub num_buckets: u32,
//    pub ofs_buckets: u32,
}

const MAGIC: u32 = 0x35444842;
named!(check_magic<&[u8], u32>, verify!(le_u32, |m| *m == MAGIC));

pub fn parse(i: &[u8]) -> IResult<&[u8], BhdHeader> {
    let (i, magic) = check_magic(i)?;
    Ok((i, BhdHeader { magic }))
}
