//! DCX format.
//!
//! Support DFLT method only.

use std::io;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

use crate::formats::common::Pack;

pub const HEADER_MAGIC: &[u8] = b"DCX\0";
pub const HEADER_SIZE: usize = 0x18;

#[derive(Debug)]
pub struct DcxHeader {
    pub magic: Vec<u8>,
    pub unk04: u32,
    pub ofs_dcs: u32,
    pub ofs_dcp: u32,
    pub unk10: u32,
    pub unk14: u32,
}

fn parse_header(i: &[u8]) -> IResult<&[u8], DcxHeader> {
    let (i, (magic, unk04, ofs_dcs, ofs_dcp, unk10, unk14)) =
        tuple((tag(HEADER_MAGIC), be_u32, be_u32, be_u32, be_u32, be_u32))(i)?;
    Ok((i, DcxHeader { magic: magic.to_vec(), unk04, ofs_dcs, ofs_dcp, unk10, unk14 }))
}

impl Pack for DcxHeader {
    fn write(&self, f: &mut dyn io::Write) -> io::Result<usize> {
        f.write_all(&self.magic)?;
        f.write_all(&self.unk04.to_be_bytes())?;
        f.write_all(&self.ofs_dcs.to_be_bytes())?;
        f.write_all(&self.ofs_dcp.to_be_bytes())?;
        f.write_all(&self.unk10.to_be_bytes())?;
        f.write_all(&self.unk14.to_be_bytes())?;
        Ok(HEADER_SIZE)
    }
}

pub const SIZES_CHUNK_MAGIC: &[u8] = b"DCS\0";
pub const SIZES_CHUNK_SIZE: usize = 0xC;

#[derive(Debug)]
pub struct DcxSizes {
    pub magic: Vec<u8>,
    pub uncompressed_size: u32,
    pub compressed_size: u32,
}

fn parse_sizes(i: &[u8]) -> IResult<&[u8], DcxSizes> {
    let (i, (magic, uncompressed_size, compressed_size)) =
        tuple((tag(SIZES_CHUNK_MAGIC), be_u32, be_u32))(i)?;
    Ok((i, DcxSizes { magic: magic.to_vec(), uncompressed_size, compressed_size }))
}

impl Pack for DcxSizes {
    fn write(&self, f: &mut dyn io::Write) -> io::Result<usize> {
        f.write_all(&self.magic)?;
        f.write_all(&self.uncompressed_size.to_be_bytes())?;
        f.write_all(&self.compressed_size.to_be_bytes())?;
        Ok(SIZES_CHUNK_SIZE)
    }
}

pub const PARAMS_CHUNK_MAGIC: &[u8] = b"DCP\0";
pub const PARAMS_CHUNK_SIZE: usize = 0x32;

#[derive(Debug)]
pub struct DcxParams {
    pub magic: Vec<u8>,
    pub method: Vec<u8>,
    pub ofs_dca: u32,
    pub unk0C: u8,
    pub unk0D: u8,
    pub unk0E: u8,
    pub unk0F: u8,
    pub unk10: u32,
    pub unk14: u32,
    pub unk18: u32,
    pub unk1C: u32,
}

fn parse_params(i: &[u8]) -> IResult<&[u8], DcxParams> {
    let (i, (magic, method, ofs_dca, flags, unk10, unk14, unk18, unk1C)) =
        tuple((
            tag(PARAMS_CHUNK_MAGIC),
            alt((tag(b"DFLT"), tag(b"EDGE"), tag(b"KRAK"))),
            be_u32,
            count(be_u8, 4),
            be_u32,
            be_u32,
            be_u32,
            be_u32,
        ))(i)?;
    Ok((
        i,
        DcxParams {
            magic: magic.to_vec(),
            method: method.to_vec(),
            ofs_dca,
            unk0C: flags[0],
            unk0D: flags[1],
            unk0E: flags[2],
            unk0F: flags[3],
            unk10,
            unk14,
            unk18,
            unk1C,
        }
    ))
}

impl Pack for DcxParams {
    fn write(&self, f: &mut dyn io::Write) -> io::Result<usize> {
        f.write_all(&self.magic)?;
        f.write_all(&self.method)?;
        f.write_all(&self.ofs_dca.to_be_bytes())?;
        f.write_all(&self.unk0C.to_be_bytes())?;
        f.write_all(&self.unk0D.to_be_bytes())?;
        f.write_all(&self.unk0E.to_be_bytes())?;
        f.write_all(&self.unk0F.to_be_bytes())?;
        f.write_all(&self.unk10.to_be_bytes())?;
        f.write_all(&self.unk14.to_be_bytes())?;
        f.write_all(&self.unk18.to_be_bytes())?;
        f.write_all(&self.unk1C.to_be_bytes())?;
        Ok(PARAMS_CHUNK_SIZE)
    }
}

pub const ARCHIVE_CHUNK_MAGIC: &[u8] = b"DCA\0";
pub const ARCHIVE_CHUNK_SIZE: usize = 0x8;

#[derive(Debug)]
pub struct DcxArchive {
    pub magic: Vec<u8>,
    pub ofs_data: u32,
}

fn parse_archive(i: &[u8]) -> IResult<&[u8], DcxArchive> {
    let (i, (magic, ofs_data)) = tuple((tag(ARCHIVE_CHUNK_MAGIC), be_u32))(i)?;
    Ok((i, DcxArchive { magic: magic.to_vec(), ofs_data }))
}

impl Pack for DcxArchive {
    fn write(&self, f: &mut dyn io::Write) -> io::Result<usize> {
        f.write_all(&self.magic)?;
        f.write_all(&self.ofs_data.to_be_bytes())?;
        Ok(ARCHIVE_CHUNK_SIZE)
    }
}

#[derive(Debug)]
pub struct Dcx {
    pub header: DcxHeader,
    pub sizes: DcxSizes,
    pub params: DcxParams,
    pub archive: DcxArchive,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Dcx> {
    let full_file = i;
    let (_, header) = parse_header(&full_file)?;
    let pos_dcs = header.ofs_dcs as usize;
    let (_, sizes) = parse_sizes(&full_file[pos_dcs..])?;
    let pos_dcp = header.ofs_dcp as usize;
    let (_, params) = parse_params(&full_file[pos_dcp..])?;
    let pos_dca = pos_dcp + params.ofs_dca as usize;
    let (i, archive) = parse_archive(&full_file[pos_dca..])?;
    Ok((i, Dcx { header, sizes, params, archive }))
}
