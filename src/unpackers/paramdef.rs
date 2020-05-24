use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::formats::paramdef;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

/// Load a PARAMDEF file from disk.
///
/// Wraps around `load_paramdef` to load the PARAMDEF from disk.
pub fn load_paramdef_file(paramdef_path: &str) -> Result<paramdef::Paramdef, UnpackError> {
    let paramdef_data = utils_fs::open_file_to_vec(path::Path::new(paramdef_path))?;
    Ok(load_paramdef(&paramdef_data)?)
}

/// Load a PARAMDEF file from a byte slice.
pub fn load_paramdef(paramdef_data: &[u8]) -> Result<paramdef::Paramdef, UnpackError> {
    match paramdef::parse(paramdef_data) {
        Ok((_, result)) => Ok(result),
        Err(NomError(e)) | Err(NomFailure(e)) => Err(UnpackError::parsing_err("PARAMDEF", e.1)),
        Err(e) => Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    }
}

/// Print verbose data about a PARAMDEF.
pub fn print_paramdef(paramdef: &paramdef::Paramdef) {
    println!("{}", paramdef);
    for field in &paramdef.fields {
        println!("  - {}", field);
        println!(
            "    Values: default {}, range [{}, {}], inc {}",
            field.default_value,
            field.min_value,
            field.max_value,
            field.increment
        );
        if let Some(desc) = &field.description {
            println!("    Description: {}", desc);
        }
    }
}
