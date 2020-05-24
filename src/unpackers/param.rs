use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::formats::param;
use crate::formats::paramdef;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

/// Load a PARAM file from disk.
///
/// Wraps around `load_param` to load the PARAM from disk.
pub fn load_param_file(
    param_path: &str,
    paramdef: Option<&paramdef::Paramdef>
) -> Result<param::Param, UnpackError> {
    let param_data = utils_fs::open_file_to_vec(path::Path::new(param_path))?;
    Ok(load_param(&param_data, paramdef)?)
}

/// Load a PARAM from a byte slice.
///
/// If paramdef is provided, it copies the right amount of bytes into
/// row data, without parsing them. Else it loads the PARAM with
/// empty row data.
pub fn load_param(
    param_data: &[u8],
    paramdef: Option<&paramdef::Paramdef>
) -> Result<param::Param, UnpackError> {
    match param::parse(param_data, paramdef) {
        Ok((_, result)) => Ok(result),
        Err(NomError(e)) | Err(NomFailure(e)) => Err(UnpackError::parsing_err("PARAM", e.1)),
        Err(e) => Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    }
}

/// Print simple information about a PARAM.
pub fn print_param(param: &param::Param) {
    println!("{}", param);
    for row in &param.rows {
        println!("  - {}", row);
        if row.data.len() > 0 {
            println!("    {:?}", row.data);
        }
    }
}

/// Print a PARAM's data using PARAMDEF fields.
pub fn print_param_with_def(param: &param::Param, paramdef: &paramdef::Paramdef) {
    println!("{}", param);
    for row in &param.rows {
        println!("  - {}", row);
        let mut desc = String::with_capacity(row.data.len() * 32);  // Rough estimate.
        for (value, field_def) in row.data.iter().zip(paramdef.fields.iter()) {
            desc.push_str(&format!("    - {}  =  {}\n", field_def, value));
        }
        println!("{}", desc.as_str());
    }
}
