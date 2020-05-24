use std::fs;
use std::io::Write;
use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::formats::bhf;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

/// Extract BHF file and corresponding BDT contents to disk.
///
/// Wraps around `extract_bhf` to load the BHF file from disk.
pub fn extract_bhf_file(
    bhf_path: &str,
    output_dir: &str,
    overwrite: bool
) -> Result<(), UnpackError> {
    let bhf = load_bhf_file(bhf_path)?;

    let bdt_path: path::PathBuf = if let Some(path) = get_bdt_for_bhf(bhf_path) {
        if !path.exists() {
            return Err(UnpackError::Naming(format!("Can't find BDT: {:?}", path)))
        }
        path
    } else {
        return Err(UnpackError::Naming(format!("Can't find BDT for BHF: {}", bhf_path)))
    };
    let bdt_data = utils_fs::open_file_to_vec(&bdt_path)?;

    extract_bhf(&bhf, &bdt_data, output_dir, overwrite)?;
    Ok(())
}

/// Return corresponding BDT path for a BHF path.
///
/// Replaces the "bhd" suffix in extension wiuth "bdt". If the
/// extension does not ends with "bhd", a warning is printed but does
/// the extension is still appended with "bdt". If the path does not
/// have an extension or is overall invalid, it returns None.
/// It does not check if the file exists either.
pub fn get_bdt_for_bhf(bhf_path: &str) -> Option<path::PathBuf> {
    let mut path = path::PathBuf::from(bhf_path);
    let ext = path.extension()?.to_str()?;
    if !ext.ends_with("bhd") {
        eprintln!("BHF extension does not end with bhd: {}", ext);
    }
    let bdtext = String::from(ext.trim_end_matches("bhd")) + "bdt";
    path.set_extension(bdtext);
    Some(path)
}

/// Extract BHF+BDT contents to disk.
///
/// Files are written in output_dir, creating it if needed, without
/// preserving directory structure.
pub fn extract_bhf(
    bhf: &bhf::Bhf,
    bdt_data: &Vec<u8>,
    output_dir: &str,
    overwrite: bool,
) -> Result<(), UnpackError> {
    let output_dir = path::Path::new(output_dir);
    utils_fs::ensure_dir_exists(output_dir)?;
    for file_info in &bhf.file_infos {
        // Extract all entries, print but ignore path errors.
        match extract_bhf_entry(file_info, bdt_data, output_dir, overwrite) {
            Err(UnpackError::Naming(e)) => { eprintln!("{}", e) }
            _ => {}
        }
    }
    Ok(())
}


/// Extract a file contained in a BHF+BDT using its BhfFileInfo.
///
/// The info struct must have a valid internal path.
pub fn extract_bhf_entry(
    file_info: &bhf::BhfFileInfo,
    bdt_data: &Vec<u8>,
    output_dir: &path::Path,
    overwrite: bool,
) -> Result<(), UnpackError> {
    if file_info.path.is_none() {
        return Err(UnpackError::Naming("No path for BHF entry.".to_owned()))
    }

    let ofs_start = file_info.ofs_data as usize;
    let ofs_end = ofs_start + file_info.size as usize;
    let data = &bdt_data[ofs_start..ofs_end];

    let internal_path = file_info.path.to_owned().unwrap();
    let file_name = internal_path.trim_start_matches('\\');
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

/// Load a BHF file from disk.
///
/// Wraps around `load_bhf` to load the BHF from disk.
pub fn load_bhf_file(bhf_path: &str) -> Result<bhf::Bhf, UnpackError> {
    let bhf_data = utils_fs::open_file_to_vec(path::Path::new(bhf_path))?;
    load_bhf(&bhf_data)
}

/// Load a BHF file from a byte slice.
pub fn load_bhf(bhf_data: &[u8]) -> Result<bhf::Bhf, UnpackError> {
    match bhf::parse(&bhf_data) {
        Ok((_, bhf)) => Ok(bhf),
        Err(NomError(e)) | Err(NomFailure(e)) => Err(UnpackError::parsing_err("BHF", e.1)),
        e => Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bdt_for_bhf() {
        assert_eq!(
            get_bdt_for_bhf("/map/m10/GI_Env_m10.tpfbhd").unwrap(),
            path::PathBuf::from("/map/m10/GI_Env_m10.tpfbdt")
        );
        // Weird case but why not.
        assert_eq!(
            get_bdt_for_bhf("/map/m10/GI_Env_m10.xxx").unwrap(),
            path::PathBuf::from("/map/m10/GI_Env_m10.xxxbdt")
        );

        assert!(get_bdt_for_bhf("").is_none());
        assert!(get_bdt_for_bhf("/map/m10/GI_Env_m10").is_none());
    }
}
