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
    pub num_entries: u16,
    pub entry_size: u16,
    pub param_name: String,
    pub endianness: u8,
    pub unicode: u8,
    pub format_version: u16,
    pub ofs_entries: u64,
}

impl ParamdefHeader {
    pub fn use_be(&self) -> bool { use_be(self.endianness) }
    pub fn has_ofs_entries(&self) -> bool { has_ofs_entries(self.format_version) }
    pub fn has_64b_ofs_desc(&self) -> bool { self.format_version >= 201 }
}

fn use_be(endianness: u8) -> bool { endianness == 0xFF }

fn has_ofs_entries(format_version: u16) -> bool { format_version >= 201 }

fn parse_header(i: &[u8]) -> IResult<&[u8], ParamdefHeader> {
    let p_u32 = if use_be(i[0x2C]) { be_u32 } else { le_u32 };
    let p_u16 = if use_be(i[0x2C]) { be_u16 } else { le_u16 };
    let (i, (file_size, header_size, data_version, num_entries, entry_size)) =
        tuple((p_u32, p_u16, p_u16, p_u16, p_u16))(i)?;
    let (i, param_name) = take_cstring_from(i, 0x20)?;
    let (i, (endianness, unicode, format_version)) =
        tuple((le_u8, le_u8, p_u16))(i)?;

    let (i, ofs_entries) = if has_ofs_entries(format_version) {
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
            num_entries,
            entry_size,
            param_name: String::from_utf8_lossy(param_name).to_string(),
            endianness,
            unicode,
            format_version,
            ofs_entries,
        }
    ))
}

pub struct ParamdefEntry {
    pub display_name: String,
    pub display_type: String,
    pub display_format: String,
    pub default_value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub increment: f32,
    pub edit_flags: u32,
    pub byte_count: u32,
    pub ofs_desc: ParamdefEntryDescOffset,
    pub internal_type: String,
    pub internal_name: Option<String>,
    pub sort_id: u32,

    pub description: Option<String>,
}

pub union ParamdefEntryDescOffset {
    ofs32: u32,
    ofs64: u64,
}

fn parse_entry<'a>(i: &'a[u8], header: &ParamdefHeader) -> IResult<&'a[u8], ParamdefEntry> {
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
        (i, ParamdefEntryDescOffset { ofs32: o })
    } else {
        let (i, o) = p_u64(i)?;
        (i, ParamdefEntryDescOffset { ofs64: o })
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
        ParamdefEntry {
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
    pub entries: Vec<ParamdefEntry>,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Paramdef> {
    let full_file = i;
    let (i, header) = parse_header(i)?;
    let i = if header.has_ofs_entries() {
        let ofs_entries = header.ofs_entries as usize;
        &full_file[ofs_entries..]
    } else {
        i  // Unsure if header.header_size has to be used here, pray there never is padding.
    };
    let (i, mut entries) = count(|i| parse_entry(i, &header), header.num_entries as usize)(i)?;

    for entry in &mut entries {
        let ofs: usize = if header.has_64b_ofs_desc() {
            unsafe { entry.ofs_desc.ofs64 as usize }
        } else {
            unsafe { entry.ofs_desc.ofs32 as usize }
        };
        if ofs == 0 {
            continue
        }
        let (_, sjis_desc) = take_cstring(&full_file[ofs..])?;
        entry.description = Some(sjis_to_string_lossy(sjis_desc));
    }

    Ok((i, Paramdef { header, entries }))
}
