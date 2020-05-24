use nom::IResult;
use nom::bytes::complete::{tag, take};
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::formats::bnd::{BinderOptions, format, use_be};
use crate::formats::common::{sjis_to_string, take_cstring};

#[derive(Debug)]
pub struct BhfHeader {
    pub magic: Vec<u8>,
    pub version: Vec<u8>,
    pub raw_format: u8,
    pub endianness: u8,  // Unsure if byte or bit endianness or both.
    pub unk0E: u8,
    pub unk0F: u8,
    pub num_files: u32,
    pub unk14: u32,
    pub unk18: u32,
    pub unk1C: u32,
}

impl BinderOptions for BhfHeader {
    /// See `formats::bnd::format` function.
    fn format(&self) -> u8 { format(self.endianness, self.raw_format) }

    /// See `formats::bnd::use_be` function.
    fn use_be(&self) -> bool { use_be(self.endianness, self.format()) }
}

fn parse_header(i: &[u8]) -> IResult<&[u8], BhfHeader> {
    let (i, (magic, version, raw_format, endianness, u8_unks)) =
        tuple((tag(b"BHF3"), take(8usize), le_u8, le_u8, count(le_u8, 2)))(i)?;
    let format = format(endianness, raw_format);
    let u32_parser = if use_be(endianness, format) { be_u32 } else { le_u32 };
    let (i, (num_files, last_unks)) =
        tuple((u32_parser, count(u32_parser, 3)))(i)?;
    Ok((
        i,
        BhfHeader {
            magic: magic.to_vec(),
            version: version.to_vec(),
            raw_format,
            endianness,
            unk0E: u8_unks[0],
            unk0F: u8_unks[1],
            num_files,
            unk14: last_unks[0],
            unk18: last_unks[1],
            unk1C: last_unks[2],
        }
    ))
}

#[derive(Debug)]
pub struct BhfFileInfo {
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

fn parse_file_info<'a>(i: &'a[u8], header: &BhfHeader) -> IResult<&'a[u8], BhfFileInfo> {
    let u32_parser = if header.use_be() { be_u32 } else { le_u32 };
    let (i, (u8_unks, size, ofs_data)) = tuple((count(le_u8, 4), u32_parser, u32_parser))(i)?;

    let (i, id) = if header.has_ids() { u32_parser(i)? } else { (i, 0) };
    let (i, ofs_path) = if header.has_paths() { u32_parser(i)? } else { (i, 0) };
    let (i, uncompressed_size) = if header.has_uncomp_size() { u32_parser(i)? } else { (i, 0) };

    Ok((
        i,
        BhfFileInfo {
            unk00: u8_unks[0],
            unk01: u8_unks[1],
            unk02: u8_unks[2],
            unk03: u8_unks[3],
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
pub struct Bhf {
    pub header: BhfHeader,
    pub file_infos: Vec<BhfFileInfo>,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Bhf> {
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
            info.path = sjis_to_string(sjis_path);
            if info.path.is_none() {
                eprintln!("Failed to parse path: {:?}", sjis_path);
            }
        }
    }
    Ok((full_file, Bhf { header, file_infos }))
}
