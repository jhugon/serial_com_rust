use crc_any::CRC;
use std::convert::TryFrom;

use crate::error::SerialComResult;

/// Trait to calculate CRCs
///
/// Implemented for:
///
///  arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping>
///
pub trait CRCExt {
    /// Compute the CRC
    ///
    /// Assumes self is some kind of buffer containing the message
    ///
    /// msg_len is the number of elements or size of buffer to compute the CRC on
    ///
    /// returns SerialComResult containing CRC
    ///
    fn compute_crc(&mut self, msg_len: &usize) -> SerialComResult<u16>;
    /// Compute the CRC, return as high and low bytes
    ///
    /// Assumes self is some kind of buffer containing the message
    ///
    /// msg_len is the number of elements or size of buffer to compute the CRC on
    ///
    /// returns SerialComResult containing a tuple of two bytes. The .0 byte is the upper 8 bits of
    /// the CRC. The .1 byte is the lower 8 bits of the CRC.
    ///
    fn compute_crc_bytes(&mut self, msg_len: &usize) -> SerialComResult<(u8, u8)>;
}

impl CRCExt for arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> {
    fn compute_crc(&mut self, msg_len: &usize) -> SerialComResult<u16> {
        let mut crc16 = CRC::crc16dnp();
        let (slice1, slice2) = self.as_slices();
        let slice1_len = slice1.len();
        if *msg_len <= slice1_len {
            crc16.digest(&slice1[0..*msg_len]);
        } else {
            crc16.digest(&slice1);
            crc16.digest(&slice2[0..(*msg_len - slice1_len)]);
        }
        let crc_num: u16 = u16::try_from(crc16.get_crc())?;
        Ok(crc_num)
    }
    fn compute_crc_bytes(&mut self, msg_len: &usize) -> SerialComResult<(u8, u8)> {
        let crc_num: u16 = self.compute_crc(msg_len)?;
        let crc_high_byte = u8::try_from(crc_num >> 8 & 0xFF)?;
        let crc_low_byte = u8::try_from(crc_num & 0xFF)?;
        Ok((crc_high_byte, crc_low_byte))
    }
}

impl CRCExt for arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> {
    fn compute_crc(&mut self, msg_len: &usize) -> SerialComResult<u16> {
        let mut crc16 = CRC::crc16dnp();
        let (slice1, slice2) = self.as_slices();
        let slice1_len = slice1.len();
        if *msg_len <= slice1_len {
            crc16.digest(&slice1[0..*msg_len]);
        } else {
            crc16.digest(&slice1);
            crc16.digest(&slice2[0..(*msg_len - slice1_len)]);
        }
        let crc_num: u16 = u16::try_from(crc16.get_crc())?;
        Ok(crc_num)
    }
    fn compute_crc_bytes(&mut self, msg_len: &usize) -> SerialComResult<(u8, u8)> {
        let crc_num: u16 = self.compute_crc(msg_len)?;
        let crc_high_byte = u8::try_from(crc_num >> 8 & 0xFF)?;
        let crc_low_byte = u8::try_from(crc_num & 0xFF)?;
        Ok((crc_high_byte, crc_low_byte))
    }
}
