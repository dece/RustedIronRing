use nom::IResult;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::parsers::common::{sjis_to_string, take_cstring_from};
use crate::utils::bin::has_flag;

const FLAGS2D_UNK1: u8          = 0b00000001;
const FLAGS2D_32B_OFS_DATA: u8  = 0b00000010;
const FLAGS2D_64B_OFS_DATA: u8  = 0b00000100;
const FLAGS2D_OFS_STRING: u8    = 0b10000000;
const FLAGS2E_UNICODE_NAMES: u8 = 0b00000001;

pub struct ParamHeader {
    pub ofs_strings: u32,
    pub ofs_data: u16,
    pub unk06: u16,
    pub paramdef_data_version: u16,
    pub num_rows: u16,
    pub param_type: String,
    pub endianness: u8,
    pub flags2D: u8,
    pub flags2E: u8,
    pub paramdef_format_version: u8,
    pub ofs_data_long: Option<ParamHeaderLongOfsData>,
}

pub union ParamHeaderLongOfsData {
    ofs32: u32,
    ofs64: u64,
}

impl ParamHeader {
    pub fn use_be(&self) -> bool { use_be(self.endianness) }
}

fn use_be(endianness: u8) -> bool { endianness == 0xFF }

fn has_ofs_string_name(flags: u8) -> bool { has_flag(flags, FLAGS2D_OFS_STRING) }

fn has_u32_ofs_data(flags: u8) -> bool { has_flag(flags, FLAGS2D_UNK1 & FLAGS2D_32B_OFS_DATA) }

fn has_u64_ofs_data(flags: u8) -> bool { has_flag(flags, FLAGS2D_64B_OFS_DATA) }

fn parse_header(i: &[u8]) -> IResult<&[u8], ParamHeader> {
    let endianness = i[0x2C];
    let p_u16 = if use_be(endianness) { be_u16 } else { le_u16 };
    let p_u32 = if use_be(endianness) { be_u32 } else { le_u32 };
    let p_u64 = if use_be(endianness) { be_u64 } else { le_u64 };
    let flags2D = i[0x2D];
    let use_u32_ofs_data = has_u32_ofs_data(flags2D);
    let use_u64_ofs_data = has_u64_ofs_data(flags2D);

    let (i, (ofs_strings, ofs_data, unk06, paramdef_data_version, num_rows)) =
        tuple((p_u32, p_u16, p_u16, p_u16, p_u16))(i)?;

    let (i, param_type) = if has_ofs_string_name(flags2D) {
        // Name is in the strings block, parse it later.
        (&i[0x20..], String::new())
    } else {
        take_cstring_from(i, 0x20).map(|(i, s)| (i, String::from_utf8_lossy(s).to_string()))?
    };

    let i = &i[0x2..];  // Skip endianness and flags2D.
    let (i, (flags2E, paramdef_format_version)) = tuple((le_u8, le_u8))(i)?;

    let (i, ofs_data_long) = if use_u32_ofs_data {
        let (_, o) = p_u32(i)?;
        (&i[0x20..], Some(ParamHeaderLongOfsData { ofs32: o }))
    } else if use_u64_ofs_data {
        let (_, o) = p_u64(i)?;
        (&i[0x20..], Some(ParamHeaderLongOfsData { ofs64: o }))
    } else {
        (i, None)
    };

    Ok((
        i,
        ParamHeader {
            ofs_strings,
            ofs_data,
            unk06,
            paramdef_data_version,
            num_rows,
            param_type,
            endianness,
            flags2D,
            flags2E,
            paramdef_format_version,
            ofs_data_long,
        }
    ))
}

pub struct Param {
    pub header: ParamHeader,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Param> {
    let full_file = i;
    let (i, header) = parse_header(i)?;
    Ok((i, Param { header }))
}
