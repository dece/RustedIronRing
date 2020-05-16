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
    println!("{} -- ver. {} -- format ver. {} -- {} fields",
        paramdef.header.param_name,
        paramdef.header.data_version, paramdef.header.format_version,
        paramdef.header.num_fields);

    for field in &paramdef.fields {
        println!("  - [{}] {} ({}) {} ({}, {} bytes)",
            field.sort_id, field.display_name,
            field.internal_name.as_ref().unwrap_or(&String::from("<noname>")),
            field.display_type, field.internal_type, field.byte_count);
        println!("    Values: default {}, range [{}, {}], inc {}",
            field.default_value, field.min_value, field.max_value, field.increment);
        if let Some(desc) = &field.description {
            println!("    Description: {}", desc);
        }
        println!("    Edit flags: {:X}", field.edit_flags);
    }
}
