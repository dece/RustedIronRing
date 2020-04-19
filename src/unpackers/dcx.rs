use std::fs;
use std::io::{Read};

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::dcx;
use crate::unpackers::errors::{self as unpackers_errors, UnpackError};

pub fn extract_dcx(dcx_path: &str, output_path: &str) -> Result<(), UnpackError> {
    let mut dcx_file = fs::File::open(dcx_path)?;
    let file_len = dcx_file.metadata()?.len() as usize;
    let mut dcx_data = vec![0u8; file_len];
    dcx_file.read_exact(&mut dcx_data)?;
    let dcx = match dcx::parse(&dcx_data) {
        Ok((_, dcx)) => { dcx }
        Err(NomError(e)) | Err(NomFailure(e)) => {
            let reason = unpackers_errors::get_nom_error_reason(e.1);
            return Err(UnpackError::Parsing("DCX parsing failed: ".to_owned() + &reason))
        }
        e => {
            return Err(UnpackError::Unknown(format!("Unknown error: {:?}", e)))
        }
    };


    println!("{:?}", dcx);
    Ok(())
}

pub fn decompress_dcx(dcx: &dcx::Dcx) -> Vec<u8> {
    let mut data = vec![0u8; dcx.sizes.uncompressed_size as usize];
    data
}
