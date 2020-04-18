use std::fs;
use std::io::{self, Read};

use crate::parsers::dcx;

pub fn extract_dcx(dcx_path: &str) -> Result<(), io::Error> {
    let mut dcx_file = fs::File::open(dcx_path)?;
    let file_len = dcx_file.metadata()?.len() as usize;
    let mut dcx_data = vec![0u8; file_len];
    dcx_file.read_exact(&mut dcx_data)?;
    dcx::parse(&dcx_data);
    Ok(())
}
