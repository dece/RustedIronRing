extern crate nom;

#[derive(Debug)]
pub struct BhdHeader {
    pub magic: u8[4];
    pub unk04: i8;
    pub unk05: i8;
    pub unk06: i8;
    pub unk07: i8;
    pub unk08: u32;
    pub num_buckets: u32;
    pub ofs_buckets: u32;
}
