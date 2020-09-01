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

/// Trait for circular buffer helper functions useful for serial communications
///
/// Implemented for:
///
///  arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping>
///
pub trait CircBufExt {
    /// Pretty print circular buffer
    fn print(&self);
    /// Push back n random elements
    ///
    /// perc_extra_zero is the percentage of elements that will be 0, in addition to the number that
    /// would be 0 from the uniform distribution.
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32);
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }

    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
}

/// Trait to do consistent overhead byte stuffing (COBS)
///
/// Implemented for:
///
///  arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping>
///
pub trait COBSExt {
    /// Encode data using consistent overhead byte stuffing (COBS)
    ///
    /// Assumes everything in the buffer is a single unencoded message, and message is < 254 bytes long
    ///
    /// returns SerialComResult with queue size;
    ///
    fn cobs_encode(&mut self) -> SerialComResult<usize>;
    /// Encode data using consistent overhead byte stuffing (COBS)
    ///
    /// Assumes buffer starts with a message ending in a 0 as a comma character, and message is < 254 bytes long
    ///
    /// Leaves the trailing 0 comma character.
    ///
    /// returns SerialComResult with size of front message on queue.
    ///
    fn cobs_decode(&mut self) -> SerialComResult<usize>;
}

impl COBSExt for arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping> {
    fn cobs_encode(&mut self) -> SerialComResult<usize> {
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_front(0u8);
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_back(0u8);
        let mut i_last_zero: usize = 0;
        let qlen = self.len();
        for i in 1..qlen {
            if let Some(&0) = self.get(i) {
                let last_zero_el = self
                    .get_mut(i_last_zero)
                    .ok_or(SerialComError::QueueIndexingError)?;
                let last_zero_el_next = u8::try_from(i - i_last_zero)?;
                *last_zero_el = last_zero_el_next;
                i_last_zero = i;
            }
        }
        Ok(self.len())
    }
    fn cobs_decode(&mut self) -> SerialComResult<usize> {
        let mut i_zero: usize = 0;
        let mut comma_found = false;
        let q_len = self.len();
        while i_zero < q_len {
            let i_zero_val = self
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
        self.pop_front();
        Ok(i_zero - 1)
    }
}

impl COBSExt for arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> {
    fn cobs_encode(&mut self) -> SerialComResult<usize> {
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_front(0u8);
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_back(0u8);
        let mut i_last_zero: usize = 0;
        let qlen = self.len();
        for i in 1..qlen {
            if let Some(&0) = self.get(i) {
                let last_zero_el = self
                    .get_mut(i_last_zero)
                    .ok_or(SerialComError::QueueIndexingError)?;
                let last_zero_el_next = u8::try_from(i - i_last_zero)?;
                *last_zero_el = last_zero_el_next;
                i_last_zero = i;
            }
        }
        Ok(self.len())
    }
    fn cobs_decode(&mut self) -> SerialComResult<usize> {
        let mut i_zero: usize = 0;
        let mut comma_found = false;
        let q_len = self.len();
        while i_zero < q_len {
            let i_zero_val = self
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
        self.pop_front();
        Ok(i_zero - 1)
    }
}

impl COBSExt for arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping> {
    fn cobs_encode(&mut self) -> SerialComResult<usize> {
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_front(0u8);
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_back(0u8);
        let mut i_last_zero: usize = 0;
        let qlen = self.len();
        for i in 1..qlen {
            if let Some(&0) = self.get(i) {
                let last_zero_el = self
                    .get_mut(i_last_zero)
                    .ok_or(SerialComError::QueueIndexingError)?;
                let last_zero_el_next = u8::try_from(i - i_last_zero)?;
                *last_zero_el = last_zero_el_next;
                i_last_zero = i;
            }
        }
        Ok(self.len())
    }
    fn cobs_decode(&mut self) -> SerialComResult<usize> {
        let mut i_zero: usize = 0;
        let mut comma_found = false;
        let q_len = self.len();
        while i_zero < q_len {
            let i_zero_val = self
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
        self.pop_front();
        Ok(i_zero - 1)
    }
}

impl COBSExt for arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> {
    fn cobs_encode(&mut self) -> SerialComResult<usize> {
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_front(0u8);
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_back(0u8);
        let mut i_last_zero: usize = 0;
        let qlen = self.len();
        for i in 1..qlen {
            if let Some(&0) = self.get(i) {
                let last_zero_el = self
                    .get_mut(i_last_zero)
                    .ok_or(SerialComError::QueueIndexingError)?;
                let last_zero_el_next = u8::try_from(i - i_last_zero)?;
                *last_zero_el = last_zero_el_next;
                i_last_zero = i;
            }
        }
        Ok(self.len())
    }
    fn cobs_decode(&mut self) -> SerialComResult<usize> {
        let mut i_zero: usize = 0;
        let mut comma_found = false;
        let q_len = self.len();
        while i_zero < q_len {
            let i_zero_val = self
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
        self.pop_front();
        Ok(i_zero - 1)
    }
}

impl COBSExt for arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping> {
    fn cobs_encode(&mut self) -> SerialComResult<usize> {
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_front(0u8);
        if self.is_full() {
            return Err(SerialComError::QueueTooFull);
        };
        self.push_back(0u8);
        let mut i_last_zero: usize = 0;
        let qlen = self.len();
        for i in 1..qlen {
            if let Some(&0) = self.get(i) {
                let last_zero_el = self
                    .get_mut(i_last_zero)
                    .ok_or(SerialComError::QueueIndexingError)?;
                let last_zero_el_next = u8::try_from(i - i_last_zero)?;
                *last_zero_el = last_zero_el_next;
                i_last_zero = i;
            }
        }
        Ok(self.len())
    }
    fn cobs_decode(&mut self) -> SerialComResult<usize> {
        let mut i_zero: usize = 0;
        let mut comma_found = false;
        let q_len = self.len();
        while i_zero < q_len {
            let i_zero_val = self
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
        self.pop_front();
        Ok(i_zero - 1)
    }
}

#[test]
fn test_cobs_encode_decode_back_8() {
    let mut rng = thread_rng();
    let mut q: arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    for _i_trial in 0..10000 {
        q.clear();
        let size = rng.gen_range(0, 6);
        q.push_back_rand(&size, &20);
        let q_orig = q.clone();
        q.cobs_encode().unwrap();
        q.cobs_decode().unwrap();
        q.pop_back(); // remove comma char
        assert_eq!(q, q_orig);
    }
}

#[test]
fn test_cobs_encode_decode_back_16() {
    let mut rng = thread_rng();
    let mut q: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    for _i_trial in 0..10000 {
        q.clear();
        let size = rng.gen_range(0, 14);
        q.push_back_rand(&size, &20);
        let q_orig = q.clone();
        q.cobs_encode().unwrap();
        q.cobs_decode().unwrap();
        q.pop_back(); // remove comma char
        assert_eq!(q, q_orig);
    }
}

#[test]
fn test_cobs_encode_decode_back_32() {
    let mut rng = thread_rng();
    let mut q: arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    for _i_trial in 0..1000 {
        q.clear();
        let size = rng.gen_range(0, 30);
        q.push_back_rand(&size, &20);
        let q_orig = q.clone();
        q.cobs_encode().unwrap();
        q.cobs_decode().unwrap();
        q.pop_back(); // remove comma char
        assert_eq!(q, q_orig);
    }
}

#[test]
fn test_cobs_encode_decode_back_64() {
    let mut rng = thread_rng();
    let mut q: arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    for _i_trial in 0..1000 {
        q.clear();
        let size = rng.gen_range(0, 62);
        q.push_back_rand(&size, &20);
        let q_orig = q.clone();
        q.cobs_encode().unwrap();
        q.cobs_decode().unwrap();
        q.pop_back(); // remove comma char
        assert_eq!(q, q_orig);
    }
}

#[test]
fn test_cobs_encode_decode_back_128() {
    let mut rng = thread_rng();
    let mut q: arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    for _i_trial in 0..1000 {
        q.clear();
        let size = rng.gen_range(0, 126);
        q.push_back_rand(&size, &20);
        let q_orig = q.clone();
        q.cobs_encode().unwrap();
        q.cobs_decode().unwrap();
        q.pop_back(); // remove comma char
        assert_eq!(q, q_orig);
    }
}
