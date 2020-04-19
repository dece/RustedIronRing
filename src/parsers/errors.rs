use nom::Err::{Error as NomError, Failure as NomFailure};

pub enum ParseError {
    Error(NomError),
    Failure(NomFailure),
}

impl From<NomError> for ParseError {
    fn from(e: NomError) -> Self {
        ParseError::Error(e)
    }
}

impl From<NomFailure> for ParseError {
    fn from(e: NomFailure) -> Self {
        ParseError::Failure(e)
    }
}
