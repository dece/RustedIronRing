use std::fs;
use std::io::{Read, Write};
use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::bnd;
use crate::unpackers::errors::{self as unpackers_errors, UnpackError};
use crate::utils::fs as fs_utils;

/// Extract BND file contents to disk.
///
/// Wraps around `extract_bnd` to load the BND from disk.
pub fn extract_bnd_file(bnd_path: &str, output_dir: &str) -> Result<(), UnpackError> {
    let (bnd, data) = load_bnd_file(bnd_path)?;
    extract_bnd(&bnd, &data, output_dir)?;
    Ok(())
}

/// Extract BND contents to disk.
///
/// Files in the BND are written in the output_path directory, creating it if needed, without
/// preserving directory structure. If the BND do not contain paths, it will be named after its ID.
/// If it does not have IDs, consecutive integers will be used.
pub fn extract_bnd(bnd: &bnd::Bnd, bnd_data: &Vec<u8>, output_dir: &str) -> Result<(), UnpackError> {
    let output_dir = path::Path::new(output_dir);
    fs_utils::ensure_dir_exists(output_dir)?;
    for file_info in &bnd.file_infos {
        extract_bnd_entry(file_info, bnd_data, output_dir)?;
    }
    Ok(())
}

/// Extract a file contained in a BND using its BndFileInfo.
///
/// The info struct must have a valid internal path.
fn extract_bnd_entry(
    file_info: &bnd::BndFileInfo,
    bnd_data: &Vec<u8>,
    output_dir: &path::Path
) -> Result<(), UnpackError> {
    if file_info.path.is_none() {
        return Err(UnpackError::Naming("No path for BND entry.".to_owned()));
    }

    let ofs_start = file_info.ofs_data as usize;
    let ofs_end = (file_info.ofs_data + file_info.size) as usize;
    let data = &bnd_data[ofs_start..ofs_end];

    let internal_path = file_info.path.to_owned().unwrap();
    // For now, do not keep internal dir structure and use only file name.
    let file_name = if let Some(last_sep_index) = internal_path.rfind('\\') {
        &internal_path[last_sep_index as usize + 1usize..]
    } else {
        &internal_path
    };
    let mut file_path = output_dir.to_path_buf();
    file_path.push(file_name);

    let mut output_file = fs::File::create(file_path)?;
    output_file.write_all(&data)?;
    Ok(())
}

/// Load a BND file from disk.
///
/// Wraps around `load_bnd` to load the BND from disk. It returns the
/// parsed BND metadata and the whole file as a byte vector.
/// Wraps around `load_bnd` to load the BND from disk.
pub fn load_bnd_file(bnd_path: &str) -> Result<(bnd::Bnd, Vec<u8>), UnpackError> {
    let mut bnd_file = fs::File::open(bnd_path)?;
    let file_len = bnd_file.metadata()?.len() as usize;
    let mut bnd_data = vec![0u8; file_len];
    bnd_file.read_exact(&mut bnd_data)?;
    Ok((load_bnd(&bnd_data)?, bnd_data))
}

/// Load a BND file from a bytes slice.
pub fn load_bnd(bnd_data: &[u8]) -> Result<bnd::Bnd, UnpackError> {
    let (_, bnd) = match bnd::parse(bnd_data) {
        Ok(result) => result,
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
