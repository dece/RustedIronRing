use std::path;

use nom::Err::{Error as NomError, Failure as NomFailure};

use crate::parsers::param;
use crate::unpackers::errors::UnpackError;
use crate::utils::fs as utils_fs;

pub fn load_param_file(param_path: &str) -> Result<param::Param, UnpackError> {
    let param_data = utils_fs::open_file_to_vec(path::Path::new(param_path))?;
    Ok(load_param(&param_data)?)
}

pub fn load_param(param_data: &[u8]) -> Result<param::Param, UnpackError> {
    match param::parse(param_data) {
        Ok((_, result)) => Ok(result),
        Err(NomError(e)) | Err(NomFailure(e)) => Err(UnpackError::parsing_err("PARAM", e.1)),
        Err(e) => Err(UnpackError::Unknown(format!("Unknown error: {:?}", e))),
    }
}

pub fn print_param(param: &param::Param) {
    println!("{} -- {} rows", param.header.param_type, param.header.num_rows);
}
