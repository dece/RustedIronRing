//typedef struct {
//    int unk00; Assert(unk00 == 0);
//    int dataOffset;
//    int dataLength;
//    int unk0C; Assert(unk0C == 1);
//} Block <bgcolor=cLtGreen, optimize=false>;
//
//typedef struct {
//    char dcx[4]; Assert(dcx == "DCX\0");
//    int unk04; Assert(unk04 == 0x10000 || unk04 == 0x11000);
//    int unk08; Assert(unk08 == 0x18);
//    int unk0C; Assert(unk0C == 0x24);
//    int unk10; Assert(unk10 == 0x24 || unk10 == 0x44);
//    int unk14; // In EDGE, size from 0x20 to end of block headers
//    char dcs[4]; Assert(dcs == "DCS\0");
//    uint uncompressedSize <format=hex>;
//    uint compressedSize <format=hex>;
//    char dcp[4]; Assert(dcp == "DCP\0");
//    char format[4]; Assert(format == "DFLT" || format == "EDGE" || format == "KRAK");
//    int unk2C; Assert(unk2C == 0x20);
//    byte unk30; Assert(unk30 == 6|| unk30 == 8 || unk30 == 9);
//    byte unk31 <hidden=true>; Assert(unk31 == 0);
//    byte unk32 <hidden=true>; Assert(unk32 == 0);
//    byte unk33 <hidden=true>; Assert(unk33 == 0);
//    int unk34; Assert(unk34 == 0 || unk34 == 0x10000); // Block size for EDGE?
//    int unk38; Assert(unk38 == 0);
//    int unk3C; Assert(unk3C == 0);
//    int unk40;
//    char dca[4]; Assert(dca == "DCA\0");
//    int dcaSize; // From before "DCA" to dca end
//
//    if (format == "EDGE") {
//        char egdt[4]; Assert(egdt == "EgdT");
//        int unk50; Assert(unk50 == 0x10100);
//        int unk54; Assert(unk54 == 0x24);
//        int unk58; Assert(unk58 == 0x10);
//        int unk5C; Assert(unk5C == 0x10000);
//        int lastBlockUncompressedSize;
//        int egdtSize; // From before "EgdT" to dca end
//        int blockCount;
//        int unk6C; Assert(unk6C == 0x100000);
//        Block blocks[blockCount];
//    }
//} Header <bgcolor=cLtRed>;

use nom::IResult;
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

const HEADER_MAGIC: u32 = 0x00504344;

fn parse_header(i: &[u8]) -> IResult<&[u8], DcxHeader> {
    let (i, (magic, unk04, ofs_dcs, ofs_dcp, unk10, unk14)) = tuple((
        tag(b"DCX\0"),
        be_u32,
        be_u32,
        be_u32,
        be_u32,
        be_u32,
    ))(i)?;
    Ok((i, DcxHeader { magic: magic.to_vec(), unk04, ofs_dcs, ofs_dcp, unk10, unk14 }))
}

#[derive(Debug)]
pub struct DcxSizes {
    pub magic: u32,
    pub uncompressed_size: u32,
    pub compressed_size: u32,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct DcxParams {
    pub magic: u32,
    pub method: [u8; 4],
    pub ofs_dca: u32,
    pub unk0C: u32,
    pub unk10: u32,
    pub unk14: u32,
    pub unk18: u32,
    pub unk1C: u32,
}

#[derive(Debug)]
pub struct Dcx {
    pub header: DcxHeader,
    pub sizes: DcxSizes,
    pub params: DcxParams,
}

pub fn parse(i: &[u8]) -> IResult<&[u8], u8> {
    let (i, header) = parse_header(i).unwrap();
    println!("{:?}", header);
    Ok((i, 0))

    //Ok((i, Dcx { header: None, sizes: None, params: None }))
}
