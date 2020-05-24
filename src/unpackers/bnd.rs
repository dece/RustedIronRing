use std::fs;
use std::io::Write;
use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::formats::bnd;
use crate::unpackers::dcx::load_dcx;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

/// Extract BND file contents to disk.
///
/// Wraps around `extract_bnd` to load the BND from disk.
pub fn extract_bnd_file(
    bnd_path: &str,
    output_dir: &str,
    overwrite: bool,
    decompress: bool,
) -> Result<(), UnpackError> {
    let (bnd, bnd_data) = if decompress {
        let (_, decomp_data) = load_dcx(bnd_path)?;
        (load_bnd(&decomp_data)?, decomp_data)
    } else {
        load_bnd_file(bnd_path)?
    };
    extract_bnd(&bnd, &bnd_data, output_dir, overwrite)?;
    Ok(())
}

/// Extract BND contents to disk.
///
/// Files in the BND are written in the output_dir directory, creating
/// it if needed, without preserving directory structure.
pub fn extract_bnd(
    bnd: &bnd::Bnd,
    bnd_data: &Vec<u8>,
    output_dir: &str,
    overwrite: bool
) -> Result<(), UnpackError> {
    let output_dir = path::Path::new(output_dir);
    utils_fs::ensure_dir_exists(output_dir)?;
    for file_info in &bnd.file_infos {
        // Extract all entries, print but ignore path errors.
        match extract_bnd_entry(file_info, bnd_data, output_dir, overwrite) {
            Err(UnpackError::Naming(e)) => eprintln!("{}", e),
            _ => {}
        }
    }
    Ok(())
}

/// Extract a file contained in a BND using its BndFileInfo.
///
/// The info struct must have a valid internal path.
pub fn extract_bnd_entry(
    file_info: &bnd::BndFileInfo,
    bnd_data: &Vec<u8>,
    output_dir: &path::Path,
    overwrite: bool,
) -> Result<(), UnpackError> {
    if file_info.path.is_none() {
        return Err(UnpackError::Naming("No path for BND entry.".to_owned()))
    }

    let ofs_start = file_info.ofs_data as usize;
    let ofs_end = ofs_start + file_info.size as usize;
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
    if !overwrite && file_path.exists() {
        let existing = file_path.to_string_lossy();
        return Err(UnpackError::Naming(format!("File already exists: {}", existing)))
    }

    let mut output_file = fs::File::create(file_path)?;
    output_file.write_all(&data)?;
    Ok(())
}

/// Load a BND file from disk.
///
/// Wraps around `load_bnd` to load the BND from disk. It returns the
/// parsed BND metadata and the whole file as a byte vector.
pub fn load_bnd_file(bnd_path: &str) -> Result<(bnd::Bnd, Vec<u8>), UnpackError> {
    let bnd_data = utils_fs::open_file_to_vec(path::Path::new(bnd_path))?;
    Ok((load_bnd(&bnd_data)?, bnd_data))
}

/// Load a BND file from a bytes slice.
pub fn load_bnd(bnd_data: &[u8]) -> Result<bnd::Bnd, UnpackError> {
    match bnd::parse(bnd_data) {
        Ok((_, result)) => Ok(result),
        Err(NomError(e)) | Err(NomFailure(e)) => Err(UnpackError::parsing_err("BND", e.1)),
        e => Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    }
}
