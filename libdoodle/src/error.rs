use std::error::Error;

pub type GenericError = Box<dyn Error>;
pub type GenericResult<T> = Result<T, GenericError>;
