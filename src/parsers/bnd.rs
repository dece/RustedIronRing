use nom::IResult;
use nom::bytes::complete::{tag, take, take_until};
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::utils::bin::has_flag;

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

    // Data computed once to avoid clutter. TODO do it better
    format: u8,       // Format with correct bit order.
    use_be: bool,     // Use big-endian to parse this BND.
    has_paths: bool,  // Files have paths.
}

const FORMAT_HAS_ID: u8          = 0b00000010;
const FORMAT_HAS_NAME1: u8       = 0b00000100;
const FORMAT_HAS_NAME2: u8       = 0b00001000;
const FORMAT_HAS_UNCOMP_SIZE: u8 = 0b00100000;

impl BndHeader {
    pub fn format(bit_en: u8, raw_format: u8) -> u8 {
        if bit_en == 1 || has_flag(raw_format, 0x1) && !has_flag(raw_format, 0x80) {
            raw_format
        } else {
            raw_format.reverse_bits()
        }
    }

    pub fn use_be(en: u8, format: u8) -> bool {
        en == 1 || has_flag(format, 0x1)
    }
}

fn parse_header(i: &[u8]) -> IResult<&[u8], BndHeader> {
    let (i, (magic, version, raw_format, endianness, bit_endianness, flags0F)) =
        tuple((tag(b"BND3"), take(8usize), le_u8, le_u8, le_u8, le_u8))(i)?;
    let format = BndHeader::format(bit_endianness, raw_format);
    let use_be = BndHeader::use_be(endianness, format);
    let u32_parser = if use_be { be_u32 } else { le_u32 };
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

            format,
            use_be,
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

    pub path: String,
}

fn parse_file_info<'a>(i: &'a[u8], header: &BndHeader) -> IResult<&'a[u8], BndFileInfo> {
    let u32_parser = if header.use_be { be_u32 } else { le_u32 };
    let (i, (flags, size, ofs_data)) = tuple((count(le_u8, 4), u32_parser, u32_parser))(i)?;

    let (i, id) = if has_flag(header.format, FORMAT_HAS_ID) { u32_parser(i)? } else { (i, 0) };
    let has_name = has_flag(header.format, FORMAT_HAS_NAME1) ||
                   has_flag(header.format, FORMAT_HAS_NAME2);
    let (i, ofs_path) = if has_name { u32_parser(i)? } else { (i, 0) };
    let has_uncomp_size = has_flag(header.format, FORMAT_HAS_UNCOMP_SIZE);
    let (i, uncompressed_size) = if has_uncomp_size { u32_parser(i)? } else { (i, 0) };

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
            path: String::new(),
        }
    ))
}

#[derive(Debug)]
pub struct Bnd {
    pub header: BndHeader,
    pub file_infos: Vec<BndFileInfo>,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Bnd> {
    let full_file = i;
    let (i, header) = parse_header(i)?;
    let (i, file_infos) = count(|i| parse_file_info(i, &header), header.num_files as usize)(i)?;
    if has_flag(header.format, FORMAT_HAS_NAME1) || has_flag(header.format, FORMAT_HAS_NAME2) {
        for info in &file_infos {
            let (_, path) = take_until(b"\0")(i[info.])?;
        }
    }
    println!("{:?}", header);
    println!("{:?}", file_infos);
    Ok((i, Bnd { header, file_infos }))
}
