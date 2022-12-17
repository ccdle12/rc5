use core::{array::TryFromSliceError, fmt};

#[derive(Debug)]
pub enum Error {
    InvalidKeyLen,
    BufferOutOfBounds,
    TryFromSliceError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidKeyLen => write!(f, "Received key is an invalid length"),
            Error::BufferOutOfBounds => write!(f, "Buffer out of bounds"),
            Error::TryFromSliceError => write!(f, "Failed to slice"),
        }
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_error: TryFromSliceError) -> Self {
        Error::TryFromSliceError
    }
}

pub(crate) type Result<T> = core::result::Result<T, Error>;
