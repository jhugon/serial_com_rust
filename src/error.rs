use std::num::TryFromIntError;

// See https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html
pub type SerialComResult<T> = std::result::Result<T, SerialComError>;

#[derive(Debug)]
pub enum SerialComError {
    QueueTooFull,
    QueueIndexingError,
    COBSDecodeNoCommaFound,
    TryFromInt(TryFromIntError),
}

impl std::fmt::Display for SerialComError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SerialComError::QueueTooFull => {
                write!(f, "Queue too full, need room for overhead and comma bytes.")
            }
            SerialComError::QueueIndexingError => {
                write!(f, "Tried to index out of bounds of queue.")
            }
            SerialComError::COBSDecodeNoCommaFound => {
                write!(f, "No comma (0) byte found whie decoding message.")
            }
            SerialComError::TryFromInt(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for SerialComError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SerialComError::QueueTooFull => None,
            SerialComError::QueueIndexingError => None,
            SerialComError::COBSDecodeNoCommaFound => None,
            SerialComError::TryFromInt(ref e) => Some(e),
        }
    }
}

impl From<TryFromIntError> for SerialComError {
    fn from(err: TryFromIntError) -> SerialComError {
        SerialComError::TryFromInt(err)
    }
}
