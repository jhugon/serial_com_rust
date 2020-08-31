use rand::prelude::*;
use std::convert::TryFrom;
use std::num::TryFromIntError;

extern crate arraydeque;

trait SerialComCircBufExt {}

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

pub fn print_array_deque<A: arraydeque::Array, B: arraydeque::behavior::Behavior>(
    q: &arraydeque::ArrayDeque<A, B>,
) where
    <A as arraydeque::Array>::Item: std::fmt::Display,
{
    println!("ArrayDeque: capacity: {}, len: {}", q.capacity(), q.len());
    print!("  [");
    let lasti = q.len() - 1;
    for (i, element) in q.iter().enumerate() {
        if i == lasti {
            println!("{}]", element)
        } else {
            print!("{}, ", element)
        }
    }
}

pub fn rand_array_deque(q: &mut arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping>, n: u8) {
    let mut rng = thread_rng();
    for _i_element in 0..n {
        if rng.gen_ratio(1, 5) {
            q.push_back(0u8);
        } else {
            q.push_back(rng.gen::<u8>());
        }
    }
}

/// Encode data using consistent overhead byte stuffing (COBS)
///
/// Assumes everything in the buffer is a single unencoded message, and message is < 254 bytes long
///
/// returns SerialComResult with queue size;
///
pub fn cobs_encode(
    q: &mut arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping>,
) -> SerialComResult<usize> {
    if q.is_full() {
        return Err(SerialComError::QueueTooFull);
    };
    q.push_front(0u8);
    if q.is_full() {
        return Err(SerialComError::QueueTooFull);
    };
    q.push_back(0u8);
    let mut i_last_zero: usize = 0;
    let qlen = q.len();
    for i in 1..qlen {
        if let Some(&0) = q.get(i) {
            let last_zero_el = q
                .get_mut(i_last_zero)
                .ok_or(SerialComError::QueueIndexingError)?;
            let last_zero_el_next = u8::try_from(i - i_last_zero)?;
            *last_zero_el = last_zero_el_next;
            i_last_zero = i;
        }
    }
    Ok(q.len())
}

/// Encode data using consistent overhead byte stuffing (COBS)
///
/// Assumes buffer starts with a message ending in a 0 as a comma character, and message is < 254 bytes long
///
/// Leaves the trailing 0 comma character.
///
/// returns SerialComResult with size of front message on queue.
///
pub fn cobs_decode(
    q: &mut arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping>,
) -> SerialComResult<usize> {
    let mut i_zero: usize = 0;
    let mut comma_found = false;
    let q_len = q.len();
    while i_zero < q_len {
        let i_zero_val = q
            .get_mut(i_zero)
            .ok_or(SerialComError::QueueIndexingError)?;
        if *i_zero_val == 0u8 {
            comma_found = true;
            break;
        }
        i_zero += usize::from(*i_zero_val);
        *i_zero_val = 0u8;
    }
    if !comma_found {
        return Err(SerialComError::COBSDecodeNoCommaFound);
    };
    q.pop_front();
    Ok(i_zero - 1)
}

#[test]
fn test_cobs_encode_decode_back() {
    let mut rng = thread_rng();
    let mut q: arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    for _i_trial in 0..10000 {
        q.clear();
        let size = rng.gen_range(0, 63);
        rand_array_deque(&mut q, size);
        let q_orig = q.clone();
        cobs_encode(&mut q).unwrap();
        cobs_decode(&mut q).unwrap();
        q.pop_back(); // remove comma char
        assert_eq!(q, q_orig);
    }
}
