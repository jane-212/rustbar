use thiserror::Error;

#[derive(Error, Debug)]
pub enum IError {
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    Io(#[from] std::io::Error)
}

pub type IResult<T> = Result<T, IError>;

impl From<&str> for IError {
    fn from(cause: &str) -> Self {
        Self::Custom(cause.to_owned())
    }
}

