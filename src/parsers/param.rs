use std::fmt::{self, Debug};

use nom::IResult;
use nom::bytes::complete::take;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::parsers::common::{sjis_to_string, take_cstring, take_cstring_from, VarSizeInt};
use crate::parsers::paramdef;
use crate::utils::bin::{has_flag, mask};

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
    pub data: Vec<ParamRowValue>,
}

#[derive(strum_macros::IntoStaticStr)]
pub enum ParamRowValue {
    S8(i8), U8(u8), S16(i16), U16(u16), S32(i32), U32(u32), F32(f32), UNK(Vec<u8>)
}

impl fmt::Debug for ParamRowValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: &str = self.into();
        write!(f, "{}: {}", name, self)
    }
}

// Could be probably be done better with a macro...
impl fmt::Display for ParamRowValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamRowValue::S8(i) => fmt::Display::fmt(&i, f),
            ParamRowValue::U8(i) => fmt::Display::fmt(&i, f),
            ParamRowValue::S16(i) => fmt::Display::fmt(&i, f),
            ParamRowValue::U16(i) => fmt::Display::fmt(&i, f),
            ParamRowValue::S32(i) => fmt::Display::fmt(&i, f),
            ParamRowValue::U32(i) => fmt::Display::fmt(&i, f),
            ParamRowValue::F32(i) => fmt::Display::fmt(&i, f),
            ParamRowValue::UNK(i) => fmt::Debug::fmt(&i, f),
        }
    }
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

fn parse_row_data<'a>(
    i: &'a[u8],
    header: &ParamHeader,
    paramdef: &paramdef::Paramdef
) -> IResult<&'a[u8], Vec<ParamRowValue>> {
    let use_be = header.use_be();
    let mut data = vec!();
    let mut bitfield = 0u16;     // Current bitfield being parsed. u16 is largest handled type.
    let mut remaining_bits = 0;  // Remaining bits in bitfield.
    let mut data_slice = i;
    for field in &paramdef.fields {
        let bit_size = field.bit_size();
        let value = if bit_size == 0 {
            let (rest, value) = parse_row_value(data_slice, &field.display_type,
                                                field.byte_count as usize, use_be)?;
            data_slice = rest;
            remaining_bits = 0;
            value
        } else {
            // Bitfield parsing. If it's the first bitfield in a series, get the containing bytes
            // in the bitfield var.
            if remaining_bits == 0 {
                let (rest, bf) = take(field.byte_count as usize)(data_slice)?;
                bitfield = match field.display_type.as_str() {
                    "u8" => { remaining_bits = 8; le_u8(bf).map(|(_, v)| v as u16)? }
                    "dummy8" => { remaining_bits = 8; le_u8(bf).map(|(_, v)| v as u16)? }
                    "u16" => {
                        remaining_bits = 16;
                        (if use_be { be_u16 } else { le_u16 }) (bf) .map(|(_, v)| v)?
                    }
                    e => panic!("Unhandled PARAMDEF type {}", e),
                };
                data_slice = rest;
            }
            // Parse masked bits.
            let value = bitfield & mask(bit_size as usize) as u16;
            // Shift bitfield so next values can be parsed directly with a bitmask.
            bitfield >>= bit_size;
            remaining_bits -= bit_size;
            match field.display_type.as_str() {
                "u8" => ParamRowValue::U8(value as u8),
                "dummy8" => ParamRowValue::U8(value as u8),
                "u16" => ParamRowValue::U16(value),
                e => panic!("Unhandled PARAMDEF type {}", e),
            }
        };
        data.push(value);
    }
    Ok((i, data))
}

fn parse_row_value<'a>(
    i: &'a[u8],
    type_str: &str,
    num_bytes: usize,
    use_be: bool
) -> IResult<&'a[u8], ParamRowValue> {
    Ok(match type_str {
        "s8" => le_i8(i)
            .map(|(i, v)| (i, ParamRowValue::S8(v)))?,
        "u8" => le_u8(i)
            .map(|(i, v)| (i, ParamRowValue::U8(v)))?,
        "s16" => (if use_be { be_i16 } else { le_i16 }) (i)
            .map(|(i, v)| (i, ParamRowValue::S16(v)))?,
        "u16" => (if use_be { be_u16 } else { le_u16 }) (i)
            .map(|(i, v)| (i, ParamRowValue::U16(v)))?,
        "s32" => (if use_be { be_i32 } else { le_i32 }) (i)
            .map(|(i, v)| (i, ParamRowValue::S32(v)))?,
        "u32" => (if use_be { be_u32 } else { le_u32 }) (i)
            .map(|(i, v)| (i, ParamRowValue::U32(v)))?,
        "f32" => (if use_be { be_f32 } else { le_f32 }) (i)
            .map(|(i, v)| (i, ParamRowValue::F32(v)))?,
        _ => take(num_bytes)(i)
            .map(|(i, v)| (i, ParamRowValue::UNK(v.to_vec())))?,
    })
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
        let def = paramdef.unwrap();
        let row_size = def.row_size();
        for row in &mut rows {
            let ofs_data = row.ofs_data.u64_if(header.has_u64_ofs_data()) as usize;
            if ofs_data == 0 {
                continue
            }
            let ofs_data_end = ofs_data + row_size;
            let (_, data) = parse_row_data(&full_file[ofs_data..ofs_data_end], &header, &def)?;
            row.data = data;
        }
    }

    Ok((i, Param { header, rows }))
}
