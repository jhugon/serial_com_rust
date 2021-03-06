use std::num::TryFromIntError;
use std::sync::mpsc;

// See https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html
pub type SerialComResult<T> = std::result::Result<T, SerialComError>;

#[derive(Debug)]
pub enum SerialComError {
    QueueTooFull,
    QueueIndexingError,
    COBSDecodeNoCommaFound,
    COBSTooLittleData,
    SliceTooSmall,
    SliceTooBig,
    CRCMismatch,
    TryFromInt(TryFromIntError),
    MPSCSendErrorRegNum(mpsc::SendError<u16>),
    MPSCSendErrorRegNumVal(mpsc::SendError<(u16, u32)>),
    MPSCSendErrorStream(mpsc::SendError<(u8, Vec<u8>)>),
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
            SerialComError::COBSTooLittleData => write!(
                f,
                "Couldn't encode/decode message because message too short."
            ),
            SerialComError::SliceTooSmall => {
                write!(f, "Slice too small to hold data part of message")
            }
            SerialComError::SliceTooBig => write!(f, "Data slice too big to fit into message"),
            SerialComError::CRCMismatch => write!(f, "Received and computed CRCs don't match"),
            SerialComError::TryFromInt(ref e) => e.fmt(f),
            SerialComError::MPSCSendErrorRegNum(ref e) => e.fmt(f),
            SerialComError::MPSCSendErrorRegNumVal(ref e) => e.fmt(f),
            SerialComError::MPSCSendErrorStream(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for SerialComError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SerialComError::QueueTooFull => None,
            SerialComError::QueueIndexingError => None,
            SerialComError::COBSDecodeNoCommaFound => None,
            SerialComError::COBSTooLittleData => None,
            SerialComError::SliceTooSmall => None,
            SerialComError::SliceTooBig => None,
            SerialComError::CRCMismatch => None,
            SerialComError::TryFromInt(ref e) => Some(e),
            SerialComError::MPSCSendErrorRegNum(ref e) => Some(e),
            SerialComError::MPSCSendErrorRegNumVal(ref e) => Some(e),
            SerialComError::MPSCSendErrorStream(ref e) => Some(e),
        }
    }
}

impl From<TryFromIntError> for SerialComError {
    fn from(err: TryFromIntError) -> SerialComError {
        SerialComError::TryFromInt(err)
    }
}

impl From<mpsc::SendError<(u16, u32)>> for SerialComError {
    fn from(err: mpsc::SendError<(u16, u32)>) -> SerialComError {
        SerialComError::MPSCSendErrorRegNumVal(err)
    }
}

impl From<mpsc::SendError<u16>> for SerialComError {
    fn from(err: mpsc::SendError<u16>) -> SerialComError {
        SerialComError::MPSCSendErrorRegNum(err)
    }
}

impl From<mpsc::SendError<(u8, Vec<u8>)>> for SerialComError {
    fn from(err: mpsc::SendError<(u8, Vec<u8>)>) -> SerialComError {
        SerialComError::MPSCSendErrorStream(err)
    }
}
