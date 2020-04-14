use std::collections::HashMap;
use std::fs::File;

extern crate nom;
use nom::{IResult};
use nom::combinator::verify;
use nom::multi::count;
use nom::number::complete::*;
use nom::sequence::tuple;

#[derive(Debug)]
pub struct BhdHeader {
    pub magic: u32,
    pub unk04: i8,  // PC=0, PS3=-1
    pub unk05: i8,
    pub unk06: i8,
    pub unk07: i8,
    pub unk08: u32,
    pub file_len: u32,
    pub num_buckets: u32,
    pub ofs_buckets: u32,
}

const MAGIC: u32 = 0x35444842;

fn parse_header(i: &[u8]) -> IResult<&[u8], BhdHeader> {
    let (i, (magic, flags, unk08, file_len, num_buckets, ofs_buckets)) =
        tuple((
            verify(le_u32, |m| *m == MAGIC),
            count(le_i8, 4),
            le_u32,
            le_u32,
            le_u32,
            le_u32,
        ))(i)?;
    Ok((i, BhdHeader {
        magic,
        unk04: flags[0], unk05: flags[1], unk06: flags[2], unk07: flags[3],
        unk08,
        file_len,
        num_buckets,
        ofs_buckets,
    }))
}

#[derive(Debug)]
pub struct BhdBucketInfo {
    pub count: u32,
    pub offset: u32,
}

fn parse_bucket_info(i: &[u8]) -> IResult<&[u8], BhdBucketInfo> {
    let (i, (count, offset)) = tuple((le_u32, le_u32))(i)?;
    Ok((i, BhdBucketInfo { count, offset }))
}

#[derive(Debug)]
pub struct BhdFile {
    pub hash: u32,
    pub size: u32,
    pub offset: u64,
}

pub fn parse_file(i: &[u8]) -> IResult<&[u8], BhdFile> {
    let (i, (hash, size, offset)) = tuple((le_u32, le_u32, le_u64))(i)?;
    Ok((i, BhdFile { hash, size, offset }))
}

#[derive(Debug)]
pub struct Bhd {
    pub header: BhdHeader,
    pub bucket_infos: Vec<BhdBucketInfo>,
    pub buckets: Vec<Vec<BhdFile>>,
}

/// Parse a BHD file into a usable Bhd struct.
pub fn parse(i: &[u8]) -> IResult<&[u8], Bhd> {
    let full_file = i;
    let (i, header) = parse_header(i)?;
    let (i, bucket_infos) = count(parse_bucket_info, header.num_buckets as usize)(i)?;

    let mut buckets: Vec<Vec<BhdFile>> = vec!();
    for b in 0..header.num_buckets {
        let bucket_info = &bucket_infos[b as usize];
        let bucket_data = &full_file[bucket_info.offset as usize..];
        let (_, bucket) = count(parse_file, bucket_info.count as usize)(bucket_data)?;
        buckets.push(bucket);
    }

    Ok((i, Bhd { header, bucket_infos, buckets }))
}

/// Extract files from a BHD/BDT pair.
pub fn extract(bhd: &Bhd, bdt_file: &File, names: &HashMap<String, String>, outputpath: &str) {
    // TODO
}
