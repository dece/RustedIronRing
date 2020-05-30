use std::io;

use nom::IResult;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::formats::common::{Pack, take_cstring_from};
use crate::utils::bin as utils_bin;

pub const HEADER_SIZE: usize = 0x40;
pub const MAGIC: u32 = 0x1E048000;  // Maybe it's 2 shorts and the 1st is padding?
pub const HEADER_PAD: usize = 0x38;  // Padding after the header.

#[derive(Debug)]
pub struct DatHeader {
    pub unk00: u32,
    pub num_files: u32,
}

fn parse_header(i: &[u8]) -> IResult<&[u8], DatHeader> {
    let (i, (unk00, num_files)) = tuple((le_u32, le_u32))(i)?;
    Ok((i, DatHeader { unk00, num_files }))
}

impl Pack for DatHeader {
    fn write(&self, f: &mut dyn io::Write) -> io::Result<usize> {
        f.write_all(&self.unk00.to_le_bytes())?;
        f.write_all(&self.num_files.to_le_bytes())?;
        Ok(0x8usize)
    }
}

pub const FILE_ENTRY_SIZE: usize = 0x40;
pub const FILE_ENTRY_NAME_MAXLEN: usize = 0x34;

#[derive(Debug)]
pub struct DatFileEntry {
    pub name: String,
    pub size: u32,
    pub padded_size: u32,
    pub ofs_data: u32,
}

fn parse_file_entry(i: &[u8]) -> IResult<&[u8], DatFileEntry> {
    let (i, name) = take_cstring_from(i, FILE_ENTRY_NAME_MAXLEN)?;
    let name = String::from_utf8_lossy(name).to_string();
    let (i, (size, padded_size, ofs_data)) = tuple((le_u32, le_u32, le_u32))(i)?;
    Ok((i, DatFileEntry { name, size, padded_size, ofs_data }))
}

impl Pack for DatFileEntry {
    fn write(&self, f: &mut dyn io::Write) -> io::Result<usize> {
        let name_bytes = self.name.as_bytes();
        f.write_all(name_bytes)?;
        f.write_all(&vec![0u8; utils_bin::pad(name_bytes.len(), FILE_ENTRY_NAME_MAXLEN)])?;
        f.write_all(&self.size.to_le_bytes())?;
        f.write_all(&self.padded_size.to_le_bytes())?;
        f.write_all(&self.ofs_data.to_le_bytes())?;
        Ok(FILE_ENTRY_SIZE)
    }
}

pub const INTERNAL_PATH_SEP: char = '/';
pub const DATA_ALIGN: usize = 0x8000;

#[derive(Debug)]
pub struct Dat {
    pub header: DatHeader,
    pub files: Vec<DatFileEntry>,
}

/// Parse a DAT archive, returning it with its full file data.
pub fn parse(i: &[u8]) -> IResult<&[u8], Dat> {
    let full_file = i;
    let (_, header) = parse_header(i)?;
    let i = &full_file[HEADER_SIZE..];
    let (_, files) = count(parse_file_entry, header.num_files as usize)(i)?;
    Ok((full_file, Dat { header, files }))
}
