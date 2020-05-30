use std::io;

#[derive(Debug)]
pub enum PackError {
    Io(io::Error),
    Compression(String),
    Unknown(String),
}

impl From<io::Error> for PackError {
    fn from(e: io::Error) -> Self {
        PackError::Io(e)
    }
}
