use nom::IResult;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::parsers::common::{sjis_to_string_lossy, take_cstring, take_cstring_from};

#[derive(Debug)]
pub struct ParamdefHeader {
    pub file_size: u32,
    pub header_size: u16,
    pub data_version: u16,
    pub num_fields: u16,
    pub field_size: u16,
    pub param_name: String,
    pub endianness: u8,
    pub unicode: u8,
    pub format_version: u16,
    pub ofs_fields: u64,
}

impl ParamdefHeader {
    pub fn use_be(&self) -> bool { use_be(self.endianness) }
    pub fn has_ofs_fields(&self) -> bool { has_ofs_fields(self.format_version) }
    pub fn has_64b_ofs_desc(&self) -> bool { self.format_version >= 201 }
    pub fn can_have_bit_size(&self) -> bool { self.format_version >= 102 }
}

fn use_be(endianness: u8) -> bool { endianness == 0xFF }

fn has_ofs_fields(format_version: u16) -> bool { format_version >= 201 }

fn parse_header(i: &[u8]) -> IResult<&[u8], ParamdefHeader> {
    let p_u32 = if use_be(i[0x2C]) { be_u32 } else { le_u32 };
    let p_u16 = if use_be(i[0x2C]) { be_u16 } else { le_u16 };
    let (i, (file_size, header_size, data_version, num_entries, entry_size)) =
        tuple((p_u32, p_u16, p_u16, p_u16, p_u16))(i)?;
    let (i, param_name) = take_cstring_from(i, 0x20)?;
    let (i, (endianness, unicode, format_version)) =
        tuple((le_u8, le_u8, p_u16))(i)?;

    let (i, ofs_entries) = if has_ofs_fields(format_version) {
        let p_u64 = if use_be(i[0x2C]) { be_u64 } else { le_u64 };
        p_u64(i)?
    } else {
        (i, 0)
    };

    Ok((
        i,
        ParamdefHeader {
            file_size,
            header_size,
            data_version,
            num_fields: num_entries,
            field_size: entry_size,
            param_name: String::from_utf8_lossy(param_name).to_string(),
            endianness,
            unicode,
            format_version,
            ofs_fields: ofs_entries,
        }
    ))
}

pub struct ParamdefField {
    pub display_name: String,
    pub display_type: String,
    pub display_format: String,
    pub default_value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub increment: f32,
    pub edit_flags: u32,
    pub byte_count: u32,
    pub ofs_desc: ParamdefFieldDescOffset,
    pub internal_type: String,
    pub internal_name: Option<String>,
    pub sort_id: u32,

    pub description: Option<String>,
}

impl ParamdefField {
    /// Return the bit size for this field, or 0 if unknown.
    ///
    /// It is contained in the internal name, unsure if there is a
    /// better way to get it.
    pub fn bit_size(&self) -> usize {
        if let Some(name) = &self.internal_name {
           if !name.contains(":") {
               return 0
           }
           if let Some(bit_size_str) = name.split(":").last().and_then(|s| Some(s.trim())) {
               return bit_size_str.parse::<usize>().unwrap_or(0)
           }
        }
        0
    }
}

pub union ParamdefFieldDescOffset {
    ofs32: u32,
    ofs64: u64,
}

fn parse_field<'a>(i: &'a[u8], header: &ParamdefHeader) -> IResult<&'a[u8], ParamdefField> {
    let (i, display_name) = take_cstring_from(i, 0x40)?;
    let (i, display_type) = take_cstring_from(i, 0x8)?;
    let (i, display_format) = take_cstring_from(i, 0x8)?;

    let p_f32 = if header.endianness == 0xFF { be_f32 } else { le_f32 };
    let p_u32 = if header.endianness == 0xFF { be_u32 } else { le_u32 };
    let p_u64 = if header.endianness == 0xFF { be_u64 } else { le_u64 };

    let (i, (default_value, min_value, max_value, increment, edit_flags, byte_count)) =
        tuple((p_f32, p_f32, p_f32, p_f32, p_u32, p_u32))(i)?;

    let (i, ofs_desc) = if header.format_version < 201 {
        let (i, o) = p_u32(i)?;
        (i, ParamdefFieldDescOffset { ofs32: o })
    } else {
        let (i, o) = p_u64(i)?;
        (i, ParamdefFieldDescOffset { ofs64: o })
    };

    let (i, internal_type) = take_cstring_from(i, 0x20)?;

    let (i, internal_name): (&[u8], Option<String>) = if header.format_version >= 102 {
        take_cstring_from(i, 0x20).map(|(i, s)| (i, Some(sjis_to_string_lossy(s))))?
    } else {
        (i, None)
    };

    let (i, sort_id) = if header.format_version >= 104 { p_u32(i)? } else { (i, 0) };

    Ok((
        i,
        ParamdefField {
            display_name: sjis_to_string_lossy(display_name),
            display_type: sjis_to_string_lossy(display_type),
            display_format: sjis_to_string_lossy(display_format),
            default_value,
            min_value,
            max_value,
            increment,
            edit_flags,
            byte_count,
            ofs_desc,
            internal_type: sjis_to_string_lossy(internal_type),
            internal_name,
            sort_id,

            description: None,
        }
    ))
}

pub struct Paramdef {
    pub header: ParamdefHeader,
    pub fields: Vec<ParamdefField>,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Paramdef> {
    let full_file = i;
    let (i, header) = parse_header(i)?;
    let i = if header.has_ofs_fields() {
        let ofs_fields = header.ofs_fields as usize;
        &full_file[ofs_fields..]
    } else {
        i  // Unsure if header.header_size has to be used here, pray there never is padding.
    };
    let (i, mut fields) = count(|i| parse_field(i, &header), header.num_fields as usize)(i)?;

    for field in &mut fields {
        let ofs: usize = if header.has_64b_ofs_desc() {
            unsafe { field.ofs_desc.ofs64 as usize }
        } else {
            unsafe { field.ofs_desc.ofs32 as usize }
        };
        if ofs == 0 {
            continue
        }
        let (_, sjis_desc) = take_cstring(&full_file[ofs..])?;
        field.description = Some(sjis_to_string_lossy(sjis_desc));
    }

    Ok((i, Paramdef { header, fields }))
}
