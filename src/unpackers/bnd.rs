use std::fs;
use std::io::Read;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::bnd;
use crate::unpackers::errors::{self as unpackers_errors, UnpackError};

/// Extract BND file contents to disk.
///
/// Wraps around `extract_bnd` to load the BND from disk.
pub fn extract_bnd_file(bnd_path: &str, output_path: &str) -> Result<(), UnpackError> {
//    let bnd = load_bnd_file(bnd_path)?;
//    extract_bnd(bnd, output_path)?;
    Ok(())
}

/// Extract BND contents to disk.
pub fn extract_bnd(bnd: bnd::Bnd, output_path: &str) -> Result<(), UnpackError> {
    //TODO
    //let mut output_file = fs::File::create(output_path)?;
    //output_file.write_all(&decomp_data)?;
    Ok(())
}

/// Load a BND file from disk.
///
/// Wraps around `load_bnd` to load the BND from disk.
pub fn load_bnd_file(bnd_path: &str) -> Result<bnd::Bnd, UnpackError> {
    let mut bnd_file = fs::File::open(bnd_path)?;
    let file_len = bnd_file.metadata()?.len() as usize;
    let mut bnd_data = vec![0u8; file_len];
    bnd_file.read_exact(&mut bnd_data)?;
    load_bnd(&bnd_data)
}

/// Load a BND file from a bytes slice.
pub fn load_bnd(bnd_data: &[u8]) -> Result<bnd::Bnd, UnpackError> {
    let (_, bnd) = match bnd::parse(bnd_data) {
        Ok(result) => { result }
        Err(NomError(e)) | Err(NomFailure(e)) => {
            let reason = unpackers_errors::get_nom_error_reason(e.1);
            return Err(UnpackError::Parsing("BND parsing failed: ".to_owned() + &reason))
        }
        e => {
            return Err(UnpackError::Unknown(format!("Unknown error: {:?}", e)))
        }
    };
    Ok(bnd)
}
