use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::param;
use crate::parsers::paramdef;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;
use crate::utils::str as utils_str;

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

fn print_param_intro(param: &param::Param) {
    println!(
        "{} -- {}",
        param.header.param_type,
        utils_str::n_pluralise(param.header.num_rows as i32, "row", "rows")
    );
}

pub fn print_param_no_data(param: &param::Param) {
    print_param_intro(param);
    for row in &param.rows {
        println!("  - [{}] {}", row.id, row.name.as_ref().unwrap_or(&String::from("<noname>")));
    }
}

pub fn print_param(param: &param::Param) {
    print_param_intro(param);
    for row in &param.rows {
        println!("{:?}", row);
    }
}
