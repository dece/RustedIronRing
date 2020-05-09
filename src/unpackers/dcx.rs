use std::fs;
use std::io::{Read, Write};

use flate2::read::ZlibDecoder;
use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::dcx;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

/// Extract DCX file content to disk.
pub fn extract_dcx(dcx_path: &str, output_path: &str) -> Result<(), UnpackError> {
    let (_dcx, decomp_data) = load_dcx(dcx_path)?;
    let mut output_file = fs::File::create(output_path)?;
    output_file.write_all(&decomp_data)?;
    Ok(())
}

/// Load a DCX file in memory along with its decompressed content.
pub fn load_dcx(dcx_path: &str) -> Result<(dcx::Dcx, Vec<u8>), UnpackError> {
    let dcx_data = utils_fs::open_file_to_vec(dcx_path)?;
    let (data, dcx) = match dcx::parse(&dcx_data) {
        Ok(result) => result,
        Err(NomError(e)) | Err(NomFailure(e)) => return Err(UnpackError::parsing_err("DCX", e.1)),
        e => return Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    };

    let decomp_data = decompress_dcx(&dcx, data)?;
    Ok((dcx, decomp_data))
}

fn decompress_dcx(dcx: &dcx::Dcx, comp_data: &[u8]) -> Result<Vec<u8>, UnpackError> {
    let method: &[u8] = dcx.params.method.as_slice();
    if method == b"DFLT" {
        decompress_dcx_dflt(dcx, comp_data)
    } else {
        let method_string = match std::str::from_utf8(method) {
            Ok(s) => { String::from(s) }
            Err(_) => { format!("{:?}", method) }
        };
        Err(UnpackError::Compression(format!("Unknown method: {}", method_string)))
    }
}

fn decompress_dcx_dflt(dcx: &dcx::Dcx, comp_data: &[u8]) -> Result<Vec<u8>, UnpackError> {
    let mut data = vec![0u8; dcx.sizes.uncompressed_size as usize];
    let mut deflater = ZlibDecoder::new(comp_data);
    deflater.read_exact(&mut data)?;
    Ok(data)
}
