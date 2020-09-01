extern crate arraydeque;

#[cfg(test)]
use rand::prelude::*;
use std::convert::TryFrom;

#[cfg(test)]
use crate::circbuf::CircBufExt;
use crate::error::SerialComError;
use crate::error::SerialComResult;

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
        if self.len() == 0 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if q_len < 3 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if self.len() == 0 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if q_len < 3 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if self.len() == 0 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if q_len < 3 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if self.len() == 0 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if q_len < 3 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if self.len() == 0 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        if q_len < 3 {
            return Err(SerialComError::COBSTooLittleData);
        }
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

impl COBSExt for Vec<u8> {
    fn cobs_encode(&mut self) -> SerialComResult<usize> {
        if self.len() == 0 {
            return Err(SerialComError::COBSTooLittleData);
        }
        self.insert(0, 0u8);
        self.push(0u8);
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
        if q_len < 3 {
            return Err(SerialComError::COBSTooLittleData);
        }
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
        self.remove(0);
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
        let size = rng.gen_range(1, 6);
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
        let size = rng.gen_range(1, 14);
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
        let size = rng.gen_range(1, 30);
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
        let size = rng.gen_range(1, 62);
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
        let size = rng.gen_range(1, 126);
        q.push_back_rand(&size, &20);
        let q_orig = q.clone();
        q.cobs_encode().unwrap();
        q.cobs_decode().unwrap();
        q.pop_back(); // remove comma char
        assert_eq!(q, q_orig);
    }
}
