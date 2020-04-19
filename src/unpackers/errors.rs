use std::io;

#[derive(Debug)]
pub enum UnpackError {
    Io(io::Error),
    Parsing(String),
    Unknown(String),
}

impl From<io::Error> for UnpackError {
    fn from(e: io::Error) -> Self {
        UnpackError::Io(e)
    }
}

pub fn get_nom_error_reason(kind: nom::error::ErrorKind) -> String {
    format!("{:?} {:?}", kind, kind.description())
}
