use std::fs;
use std::io::{Read, Write};
use std::path;

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
    let dcx_path = path::Path::new(dcx_path);
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

/// Get a decompressed path for this file in this path.
///
/// If the path is some valid file path (existing or not), use it.
/// If the path is None, it tries to strip "dcx" extension from the
/// path and return it. If there is no extension, return None.
/// If the path is a valid dir path, tries to strip dcx from file name
/// and join this file name to the output path.
pub fn get_decompressed_path(dcx_path: &str, output_path: Option<&str>) -> Option<String> {
    let mut output_path_valid = false;
    // If no output path is provided, try to strip the file extension.
    let mut output_path: String = match output_path {
        Some(s) => { output_path_valid = true; s.to_string() }
        _ => { String::with_capacity(dcx_path.len()) }
    };
    if !output_path_valid {
        if let Some(pb) = utils_fs::strip_extension(&path::PathBuf::from(&dcx_path)) {
            if let Some(s) = pb.to_str() {
                output_path.push_str(s);
                output_path_valid = true;
            }
        }
    }
    if !output_path_valid {
        eprintln!("Can't determine a valid output path: {}", dcx_path);
        return None
    }
    // If the output path is a dir, try to strip extension and place the file there.
    if path::Path::new(&output_path).is_dir() {
        output_path_valid = false;
        if let Some(file_pb) = utils_fs::strip_extension(&path::PathBuf::from(&dcx_path)) {
            if let Some(file_name) = file_pb.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    let mut out_pb = path::PathBuf::from(&output_path);
                    out_pb.push(file_name_str);
                    if let Some(s) = out_pb.as_path().to_str() {
                        output_path.clear();
                        output_path.push_str(s);
                        output_path_valid = true;
                    }
                }
            }
        }
    }
    if !output_path_valid {
        eprintln!("Can't determine a valid output path: {}", dcx_path);
        return None
    }
    Some(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_decompressed_path() {
        // Without an output path.
        assert_eq!(get_decompressed_path("file.ext.dcx", None).unwrap(), "file.ext");
        assert_eq!(get_decompressed_path("file.ext", None).unwrap(), "file");
    }
}
