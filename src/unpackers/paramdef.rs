use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::paramdef;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;
use crate::utils::str as utils_str;

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

/// Print brief data about a PARAMDEF.
pub fn print_paramdef_intro(paramdef: &paramdef::Paramdef) {
    println!(
        "{} -- ver. {} -- format ver. {} -- {} fields -- {} per row",
        paramdef.header.param_name,
        paramdef.header.data_version, paramdef.header.format_version,
        paramdef.header.num_fields,
        utils_str::n_bytes_pluralise(paramdef.row_size() as i32)
    );
}

/// Print verbose data about a PARAMDEF.
pub fn print_paramdef(paramdef: &paramdef::Paramdef) {
    print_paramdef_intro(paramdef);
    for field in &paramdef.fields {
        let size_str = match field.bit_size() {
            0 => utils_str::n_bytes_pluralise(field.byte_count as i32),
            x => utils_str::n_pluralise(x as i32, "bit", "bits")
        };
        println!(
            "  - [{}] {} ({}) {} ({}, {})",
            field.sort_id,
            field.display_name,
            field.internal_name.as_ref().unwrap_or(&String::from("<noname>")),
            field.display_type,
            field.internal_type,
            size_str
        );
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
