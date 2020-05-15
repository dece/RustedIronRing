use std::str;

use nom::IResult;
use nom::bytes::complete::{tag, take};
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::parsers::common::{sjis_to_string, take_cstring};

#[derive(Debug)]
pub struct ParamdefHeader {
    pub file_size: u32,
    pub header_size: u16,
    pub data_version: u16,
    pub num_entries: u16,
    pub entry_size: u16,
    pub param_name: Vec<u8>,
    pub endianness: u8,
    pub unicode: u8,
    pub format_version: u16,
}

fn parse_header(i: &[u8]) -> IResult<&[u8], ParamdefHeader> {
    let p_u32 = if i[0x2C] == 0xFF { be_u32 } else { le_u32 };
    let p_u16 = if i[0x2C] == 0xFF { be_u16 } else { le_u16 };
    let (i, (file_size, header_size, data_version, num_entries, entry_size)) =
        tuple((p_u32, p_u16, p_u16, p_u16, p_u16))(i)?;
    let (_, param_name) = take_cstring(&i[..0x20])?;
    let (i, (endianness, unicode, format_version)) =
        tuple((le_u8, le_u8, p_u16))(&i[0x20..])?;
    Ok((
        i,
        ParamdefHeader {
            file_size,
            header_size,
            data_version,
            num_entries,
            entry_size,
            param_name: param_name.to_vec(),
            endianness,
            unicode,
            format_version,
        }
    ))
}

#[derive(Debug)]
pub struct Paramdef {
    pub header: ParamdefHeader,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Paramdef> {
    let (i, header) = parse_header(i)?;
    Ok((i, Paramdef { header }))
}
