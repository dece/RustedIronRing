use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::paramdef;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

pub fn load_paramdef_file(paramdef_path: &str) -> Result<paramdef::Paramdef, UnpackError> {
    let paramdef_data = utils_fs::open_file_to_vec(path::Path::new(paramdef_path))?;
    Ok(load_paramdef(&paramdef_data)?)
}

pub fn load_paramdef(paramdef_data: &[u8]) -> Result<paramdef::Paramdef, UnpackError> {
    match paramdef::parse(paramdef_data) {
        Ok((_, result)) => Ok(result),
        Err(NomError(e)) | Err(NomFailure(e)) => Err(UnpackError::parsing_err("PARAMDEF", e.1)),
        Err(e) => Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    }
}

pub fn print_paramdef(paramdef: &paramdef::Paramdef) {
    println!("{} -- {} entries", paramdef.header.param_name, paramdef.header.num_entries);
    println!("Data version: {}", paramdef.header.data_version);
    println!("Format version: {}", paramdef.header.format_version);

    for entry in &paramdef.entries {
        println!("  - {} ({})", entry.display_name, entry.display_type);
        if let Some(name) = &entry.internal_name {
            println!("    Internal name and type: {}, {}", name, entry.internal_type);
        }
        if let Some(desc) = &entry.description {
            println!("    Description: {}", desc);
        }
    }
}
