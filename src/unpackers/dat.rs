use std::fs;
use std::io::Write;
use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::formats::dat;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

pub fn extract_dat_file(dat_path: &str, output_path: &str) -> Result<(), UnpackError> {
    let (dat, dat_data) = load_dat_file(dat_path)?;
    extract_dat(&dat, dat_data, output_path)
}

pub fn extract_dat(
    dat: &dat::Dat,
    dat_data: Vec<u8>,
    output_path: &str
) -> Result<(), UnpackError> {
    let output_dir = path::Path::new(output_path);
    utils_fs::ensure_dir_exists(output_dir)?;
    for file_entry in &dat.files {
        match extract_file(file_entry, &dat_data, output_dir) {
            Err(UnpackError::Io(e)) => eprintln!("Can't extract {}: {}", file_entry.name, e),
            _ => {}
        }
    }
    Ok(())
}

fn extract_file(
    file_entry: &dat::DatFileEntry,
    data: &Vec<u8>,
    output_dir: &path::Path
) -> Result<(), UnpackError> {
    let ofs_start = file_entry.ofs_data as usize;
    let ofs_end = ofs_start + file_entry.size as usize;
    let data = &data[ofs_start..ofs_end];
    let internal_path = &file_entry.name;
    // If the path contains dirs, they have to be created.
    if internal_path.contains('/') {
        let internal_pb = path::PathBuf::from(internal_path);
        let internal_parent_dir = internal_pb.parent()
            .ok_or(UnpackError::Naming(format!("Bad path: {:?}", internal_pb)))?;
        let mut parent_dir = output_dir.to_path_buf();
        parent_dir.push(internal_parent_dir);
        utils_fs::ensure_dir_exists(&parent_dir)?;
    }
    let mut file_path = output_dir.to_path_buf();
    file_path.push(internal_path);
    let mut output_file = fs::File::create(file_path)?;
    output_file.write_all(&data)?;
    Ok(())
}

pub fn load_dat_file(dat_path: &str) -> Result<(dat::Dat, Vec<u8>), UnpackError> {
    let dat_data = utils_fs::open_file_to_vec(path::Path::new(dat_path))?;
    Ok((load_dat(&dat_data)?, dat_data))
}

pub fn load_dat(dat_data: &[u8]) -> Result<dat::Dat, UnpackError> {
    match dat::parse(&dat_data) {
        Ok((_, dat)) => Ok(dat),
        Err(NomError(e)) | Err(NomFailure(e)) => Err(UnpackError::parsing_err("DAT", e.1)),
        e => Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    }
}
