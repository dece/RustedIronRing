use nom::IResult;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::formats::common::take_cstring_from;

#[derive(Debug)]
pub struct DatHeader {
    pub unk00: u32,
    pub num_files: u32,
}

fn parse_header(i: &[u8]) -> IResult<&[u8], DatHeader> {
    let (i, (unk00, num_files)) = tuple((le_u32, le_u32))(i)?;
    Ok((i, DatHeader { unk00, num_files }))
}

#[derive(Debug)]
pub struct DatFileEntry {
    pub name: String,
    pub size: u32,
    pub padded_size: u32,
    pub ofs_data: u32,
}

fn parse_file_entry(i: &[u8]) -> IResult<&[u8], DatFileEntry> {
    let (i, name) = take_cstring_from(i, 0x34)?;
    let name = String::from_utf8_lossy(name).to_string();
    let (i, (size, padded_size, ofs_data)) = tuple((le_u32, le_u32, le_u32))(i)?;
    Ok((i, DatFileEntry { name, size, padded_size, ofs_data }))
}

#[derive(Debug)]
pub struct Dat {
    pub header: DatHeader,
    pub files: Vec<DatFileEntry>,
}

/// Parse a DAT archive, returning it with its full file data.
pub fn parse(i: &[u8]) -> IResult<&[u8], Dat> {
    let full_file = i;
    let (_, header) = parse_header(i)?;
    let i = &full_file[0x40..];
    let (_, files) = count(parse_file_entry, header.num_files as usize)(i)?;
    Ok((full_file, Dat { header, files }))
}
