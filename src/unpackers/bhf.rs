use std::fs;
use std::io::Read;
use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::bhf;
use crate::unpackers::errors::UnpackError;

pub fn extract_bhf(bhf_path: &str) -> Result<(), UnpackError> {
    let mut bhf_file = fs::File::open(bhf_path)?;
    let file_len = bhf_file.metadata()?.len() as usize;
    let mut bhf_data = vec![0u8; file_len];
    bhf_file.read_exact(&mut bhf_data)?;
    let bhf = match bhf::parse(&bhf_data) {
        Ok((_, bhf)) => { bhf }
        Err(NomError(e)) | Err(NomFailure(e)) => return Err(UnpackError::parsing_err("BHF", e.1)),
        e => return Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    };

    let bdt_path = get_bdt_for_bhf(bhf_path);
    if bdt_path.is_none() {
        return Err(UnpackError::Naming(format!("Can't find BDT for BHF: {}", bhf_path)))
    }
    Ok(())
}

fn get_bdt_for_bhf(bhf_path: &str) -> Option<path::PathBuf> {
    let mut path = path::PathBuf::from(bhf_path);
    let ext = path.extension()?.to_str()?;
    if !ext.ends_with("bhd") {
        eprintln!("BHF extension does not end with bhd: {}", ext);
    }
    let bdtext = String::from(ext.trim_end_matches("bhd")) + "bdt";
    path.set_extension(bdtext);
    Some(path)
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
