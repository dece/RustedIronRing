use nom::IResult;
use nom::bytes::complete::take;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::parsers::common::{sjis_to_string, take_cstring, take_cstring_from, VarSizeInt};
use crate::parsers::paramdef;
use crate::utils::bin::has_flag;

const FLAGS2D_UNK1: u8          = 0b00000001;
const FLAGS2D_32B_OFS_DATA: u8  = 0b00000010;
const FLAGS2D_64B_OFS_DATA: u8  = 0b00000100;
const FLAGS2D_OFS_STRING: u8    = 0b10000000;
const FLAGS2E_UNICODE_NAMES: u8 = 0b00000001;

#[derive(Debug)]
pub struct ParamHeader {
    pub ofs_strings: u32,  // Unreliable.
    pub ofs_data: u16,
    pub unk06: u16,
    pub paramdef_data_version: u16,
    pub num_rows: u16,
    pub param_type: String,
    pub ofs_name: Option<u64>,
    pub endianness: u8,
    pub flags2D: u8,
    pub flags2E: u8,
    pub paramdef_format_version: u8,
    pub ofs_data_long: Option<VarSizeInt>,
}

impl ParamHeader {
    pub fn use_be(&self) -> bool { use_be(self.endianness) }
    pub fn has_ofs_string_name(&self) -> bool { has_ofs_string_name(self.flags2D) }
    pub fn has_u32_ofs_data(&self) -> bool { has_u32_ofs_data(self.flags2D) }
    pub fn has_u64_ofs_data(&self) -> bool { has_u64_ofs_data(self.flags2D) }
}

fn use_be(endianness: u8) -> bool { endianness == 0xFF }

fn has_ofs_string_name(flags: u8) -> bool { has_flag(flags, FLAGS2D_OFS_STRING) }

fn has_u32_ofs_data(flags: u8) -> bool { has_flag(flags, FLAGS2D_UNK1 | FLAGS2D_32B_OFS_DATA) }

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

    let (i, param_type, ofs_name) = if has_ofs_string_name(flags2D) {
        // Name is in the strings block, parse it later.
        let (i, (_, ofs_name, _)) = tuple((take(0x4usize), p_u64, take(0x14usize)))(i)?;
        (i, String::new(), Some(ofs_name))
    } else {
        let (i, name) = take_cstring_from(i, 0x20)?;
        (i, String::from_utf8_lossy(name).to_string(), None)
    };

    let i = &i[0x2..];  // Skip endianness and flags2D.
    let (i, (flags2E, paramdef_format_version)) = tuple((le_u8, le_u8))(i)?;

    let (i, ofs_data_long) = if use_u32_ofs_data {
        let (_, o) = p_u32(i)?;
        (&i[0x20..], Some(VarSizeInt { vu32: o }))
    } else if use_u64_ofs_data {
        let (_, o) = p_u64(i)?;
        (&i[0x20..], Some(VarSizeInt { vu64: o }))
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
            ofs_name,
            endianness,
            flags2D,
            flags2E,
            paramdef_format_version,
            ofs_data_long,
        }
    ))
}

#[derive(Debug)]
pub struct ParamRow {
    pub id: u32,
    pub ofs_data: VarSizeInt,
    pub ofs_name: VarSizeInt,
    pub name: Option<String>,
    pub data: Vec<u8>,
}

fn parse_row<'a>(i: &'a[u8], header: &ParamHeader) -> IResult<&'a[u8], ParamRow> {
    let p_u32 = if header.use_be() { be_u32 } else { le_u32 };
    let p_u64 = if header.use_be() { be_u64 } else { le_u64 };

    let (i, (id, ofs_data, ofs_name)) = if header.has_u64_ofs_data() {
        let (i, (id, _, ofs_data, ofs_name)) = tuple((p_u32, take(4usize), p_u64, p_u64))(i)?;
        (i, (id, VarSizeInt { vu64: ofs_data }, VarSizeInt { vu64: ofs_name }))
    } else {
        let (i, (id, ofs_data, ofs_name)) = tuple((p_u32, p_u32, p_u32))(i)?;
        (i, (id, VarSizeInt { vu32: ofs_data }, VarSizeInt { vu32: ofs_name }))
    };

    Ok((i, ParamRow { id, ofs_data, ofs_name, name: None, data: vec!() }))
}

#[derive(Debug)]
pub struct Param {
    pub header: ParamHeader,
    pub rows: Vec<ParamRow>,
}

pub fn parse<'a>(i: &'a[u8], paramdef: Option<&paramdef::Paramdef>) -> IResult<&'a[u8], Param> {
    let full_file = i;
    let (i, mut header) = parse_header(i)?;
    if header.has_ofs_string_name() && header.ofs_name.is_some() {
        let ofs_name = header.ofs_name.unwrap() as usize;
        let (_, name) = take_cstring(&full_file[ofs_name..])?;
        header.param_type.push_str(&String::from_utf8_lossy(name).to_string());
    }

    let (i, mut rows) = count(|i| parse_row(i, &header), header.num_rows as usize)(i)?;

    for row in &mut rows {
        let ofs_name = row.ofs_name.u64_if(header.has_u64_ofs_data()) as usize;
        if ofs_name != 0 {
            let (_, name) = take_cstring(&full_file[ofs_name..])?;
            row.name = sjis_to_string(name).or_else(|| {
                eprintln!("Can't parse row name: {:?}", name);
                None
            });
        }
    }

    if paramdef.is_some() {
        let row_size = paramdef.unwrap().row_size();
        for row in &mut rows {
            let ofs_data = row.ofs_data.u64_if(header.has_u64_ofs_data()) as usize;
            if ofs_data == 0 {
                continue
            }
            let ofs_data_end = ofs_data + row_size;
            row.data = full_file[ofs_data..ofs_data_end].to_vec();
        }
    }

    Ok((i, Param { header, rows }))
}
