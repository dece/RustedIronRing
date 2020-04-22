use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

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
        tuple((tag(b"DCX\0"), be_u32, be_u32, be_u32, be_u32, be_u32))(i)?;
    Ok((i, DcxHeader { magic: magic.to_vec(), unk04, ofs_dcs, ofs_dcp, unk10, unk14 }))
}

#[derive(Debug)]
pub struct DcxSizes {
    pub magic: Vec<u8>,
    pub uncompressed_size: u32,
    pub compressed_size: u32,
}

fn parse_sizes(i: &[u8]) -> IResult<&[u8], DcxSizes> {
    let (i, (magic, uncompressed_size, compressed_size)) =
        tuple((tag(b"DCS\0"), be_u32, be_u32))(i)?;
    Ok((i, DcxSizes { magic: magic.to_vec(), uncompressed_size, compressed_size }))
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
fn parse_params(i: &[u8]) -> IResult<&[u8], DcxParams> {
    let (i, (magic, method, ofs_dca, flags, unk10, unk14, unk18, unk1C)) =
        tuple((
            tag(b"DCP\0"),
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
            unk1C
        }
    ))
}

#[derive(Debug)]
pub struct DcxArchive {
    pub magic: Vec<u8>,
    pub ofs_data: u32,
}

fn parse_archive(i: &[u8]) -> IResult<&[u8], DcxArchive> {
    let (i, (magic, ofs_data)) = tuple((tag(b"DCA\0"), be_u32))(i)?;
    Ok((i, DcxArchive { magic: magic.to_vec(), ofs_data }))
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
