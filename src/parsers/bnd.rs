use nom::IResult;
use nom::bytes::complete::{tag, take};
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::utils::bin::has_flag;
use crate::parsers::common::{sjis_to_string, take_cstring};

const FORMAT_BE: u8              = 0b00000001;
const FORMAT_HAS_ID: u8          = 0b00000010;
const FORMAT_HAS_NAME1: u8       = 0b00000100;
const FORMAT_HAS_NAME2: u8       = 0b00001000;
const FORMAT_HAS_UNCOMP_SIZE: u8 = 0b00100000;

#[derive(Debug)]
pub struct BndHeader {
    pub magic: Vec<u8>,
    pub version: Vec<u8>,
    pub raw_format: u8,
    pub endianness: u8,
    pub bit_endianness: u8,
    pub flags0F: u8,
    pub num_files: u32,
    pub ofs_data: u32,
    pub unk18: u32,
    pub unk1C: u32,
}

pub trait BinderOptions {
    fn format(&self) -> u8;
    fn use_be(&self) -> bool;

    /// Return whether files have IDs.
    fn has_ids(&self) -> bool {
        has_flag(self.format(), FORMAT_HAS_ID)
    }

    /// Return whether files have paths.
    fn has_paths(&self) -> bool {
        let format = self.format();
        has_flag(format, FORMAT_HAS_NAME1) || has_flag(format, FORMAT_HAS_NAME2)
    }

    /// Return whether files have uncompressed size.
    fn has_uncomp_size(&self) -> bool {
        has_flag(self.format(), FORMAT_HAS_UNCOMP_SIZE)
    }
}

impl BinderOptions for BndHeader {
    /// See `format` function.
    fn format(&self) -> u8 { format(self.bit_endianness, self.raw_format) }

    /// See `use_be` function.
    fn use_be(&self) -> bool { use_be(self.endianness, self.format()) }
}

/// Return format u8 with varying endianness managed.
pub fn format(bit_en: u8, raw_format: u8) -> u8 {
    if bit_en == 1 || has_flag(raw_format, FORMAT_BE) && !has_flag(raw_format, 0x80) {
        raw_format
    } else {
        raw_format.reverse_bits()
    }
}

/// Return whether parsing byte order is big endian or not.
pub fn use_be(en: u8, format: u8) -> bool {
    en == 1 || has_flag(format, FORMAT_BE)
}

fn parse_header(i: &[u8]) -> IResult<&[u8], BndHeader> {
    let (i, (magic, version, raw_format, endianness, bit_endianness, flags0F)) =
        tuple((tag(b"BND3"), take(8usize), le_u8, le_u8, le_u8, le_u8))(i)?;
    let format = format(bit_endianness, raw_format);
    let u32_parser = if use_be(endianness, format) { be_u32 } else { le_u32 };
    let (i, (num_files, ofs_data, unk18, unk1C)) =
        tuple((u32_parser, u32_parser, u32_parser, u32_parser))(i)?;
    Ok((
        i,
        BndHeader {
            magic: magic.to_vec(),
            version: version.to_vec(),
            raw_format,
            endianness,
            bit_endianness,
            flags0F,
            num_files,
            ofs_data,
            unk18,
            unk1C,
        }
    ))
}

#[derive(Debug)]
pub struct BndFileInfo {
    pub unk00: u8,
    pub unk01: u8,
    pub unk02: u8,
    pub unk03: u8,
    pub size: u32,
    pub ofs_data: u32,
    pub id: u32,
    pub ofs_path: u32,
    pub uncompressed_size: u32,

    pub path: Option<String>,
}

fn parse_file_info<'a>(i: &'a[u8], header: &BndHeader) -> IResult<&'a[u8], BndFileInfo> {
    let u32_parser = if header.use_be() { be_u32 } else { le_u32 };
    let (i, (flags, size, ofs_data)) = tuple((count(le_u8, 4), u32_parser, u32_parser))(i)?;

    let (i, id) = if header.has_ids() { u32_parser(i)? } else { (i, 0) };
    let (i, ofs_path) = if header.has_paths() { u32_parser(i)? } else { (i, 0) };
    let (i, uncompressed_size) = if header.has_uncomp_size() { u32_parser(i)? } else { (i, 0) };

    Ok((
        i,
        BndFileInfo {
            unk00: flags[0],
            unk01: flags[1],
            unk02: flags[2],
            unk03: flags[3],
            size,
            ofs_data,
            id,
            ofs_path,
            uncompressed_size,
            path: None,
        }
    ))
}

#[derive(Debug)]
pub struct Bnd {
    pub header: BndHeader,
    pub file_infos: Vec<BndFileInfo>,
}

/// Parse a BND file to a BND struct.
///
/// On success, returns the full BND data along with the Bnd struct
/// instead of the remaining data.
pub fn parse(i: &[u8]) -> IResult<&[u8], Bnd> {
    let full_file = i;
    let (i, header) = parse_header(i)?;
    let (_, mut file_infos) = count(
        |i| parse_file_info(i, &header),
        header.num_files as usize
    )(i)?;
    if header.has_paths() {
        for info in &mut file_infos {
            let ofs_path = info.ofs_path as usize;
            let (_, sjis_path) = take_cstring(&full_file[ofs_path..])?;
            info.path = sjis_to_string(sjis_path).or_else(|| {
                eprintln!("Failed to parse path: {:?}", sjis_path); None
            });
        }
    }
    Ok((full_file, Bnd { header, file_infos }))
}
