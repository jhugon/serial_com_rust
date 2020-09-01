//use crate::circbuf::CircBufExt;
use crate::cobs::COBSExt;
use crate::error::{SerialComError, SerialComResult};

use crc_any::CRC;
#[cfg(test)]
use rand::prelude::*;
use std::convert::TryFrom;

pub trait BinaryCom {
    /// Put a message in output buffer
    ///
    /// Do appropriate formatting, byte stuffing, checksum, etc.
    ///
    /// The max data buffer size is the max message size - 6. That is 1 byte overhead, 1 byte comma, 1
    /// byte version, 1 byte command, 2 bytes checksum.
    ///
    /// Returns final message length
    fn send_message(&mut self, version: &u8, command: &u8, data: &[u8]) -> SerialComResult<usize>;

    /// Read message (if one exists) from input buffer into message
    ///
    /// Do appropriate formatting, byte stuffing, checksum, etc.
    ///
    /// The data buffer must be the max message size - 6. That is 1 byte overhead, 1 byte comma, 1
    /// byte version, 1 byte command, 2 bytes checksum.
    ///
    /// Returns the received data length
    fn receive_message(
        &mut self,
        version: &mut u8,
        command: &mut u8,
        data: &mut [u8],
    ) -> SerialComResult<usize>;
}

impl BinaryCom for arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> {
    fn send_message(&mut self, version: &u8, command: &u8, data: &[u8]) -> SerialComResult<usize> {
        if data.len() > 16 - 6 {
            return Err(SerialComError::SliceTooBig);
        }
        self.clear();
        self.push_back(*version);
        self.push_back(*command);
        for el in data {
            self.push_back(*el);
        }
        let mut crc16 = CRC::crc16dnp();
        let (slice1, slice2) = self.as_slices();
        crc16.digest(slice1);
        crc16.digest(slice2);
        let crc_num: u16 = u16::try_from(crc16.get_crc())?;
        self.push_back(u8::try_from(crc_num >> 8 & 0xFF)?);
        self.push_back(u8::try_from(crc_num & 0xFF)?);
        self.cobs_encode()?;
        Ok(self.len())
    }
    fn receive_message(
        &mut self,
        version: &mut u8,
        command: &mut u8,
        data: &mut [u8],
    ) -> SerialComResult<usize> {
        if data.len() < 16 - 4 {
            return Err(SerialComError::SliceTooSmall);
        }
        let msg_chk_size = self.cobs_decode()?;
        let msg_size = msg_chk_size - 2;
        let data_size = msg_size - 2;
        let mut crc16 = CRC::crc16dnp();
        let (slice1, slice2) = self.as_slices();
        let slice1_size = slice1.len();
        if msg_size <= slice1_size {
            crc16.digest(&slice1[0..msg_size]);
        } else {
            crc16.digest(&slice1);
            crc16.digest(&slice2[0..(msg_size - slice1_size)]);
        }
        let crc_num: u16 = u16::try_from(crc16.get_crc())?;
        *version = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        *command = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        for i in 0..(data_size) {
            data[i] = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        let crc_rec_high_byte =
            u16::from(self.pop_front().ok_or(SerialComError::QueueIndexingError)?);
        let crc_rec_low_byte =
            u16::from(self.pop_front().ok_or(SerialComError::QueueIndexingError)?);
        let crc_rec: u16 = (crc_rec_high_byte << 8) & crc_rec_low_byte;
        if crc_num != crc_rec {
            return Err(SerialComError::CRCMismatch);
        }
        Ok(data_size)
    }
}

impl BinaryCom for arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> {
    fn send_message(&mut self, version: &u8, command: &u8, data: &[u8]) -> SerialComResult<usize> {
        if data.len() > 64 - 6 {
            return Err(SerialComError::SliceTooBig);
        }
        self.clear();
        self.push_back(*version);
        self.push_back(*command);
        for el in data {
            self.push_back(*el);
        }
        let mut crc16 = CRC::crc16dnp();
        let (slice1, slice2) = self.as_slices();
        crc16.digest(slice1);
        crc16.digest(slice2);
        let crc_num: u16 = u16::try_from(crc16.get_crc())?;
        self.push_back(u8::try_from(crc_num >> 8 & 0xFF)?);
        self.push_back(u8::try_from(crc_num & 0xFF)?);
        self.cobs_encode()?;
        Ok(self.len())
    }
    fn receive_message(
        &mut self,
        version: &mut u8,
        command: &mut u8,
        data: &mut [u8],
    ) -> SerialComResult<usize> {
        if data.len() < 64 - 4 {
            return Err(SerialComError::SliceTooSmall);
        }
        let msg_chk_size = self.cobs_decode()?;
        let msg_size = msg_chk_size - 2;
        let data_size = msg_size - 2;
        let mut crc16 = CRC::crc16dnp();
        let (slice1, slice2) = self.as_slices();
        let slice1_size = slice1.len();
        if msg_size <= slice1_size {
            crc16.digest(&slice1[0..msg_size]);
        } else {
            crc16.digest(&slice1);
            crc16.digest(&slice2[0..(msg_size - slice1_size)]);
        }
        let crc_num: u16 = u16::try_from(crc16.get_crc())?;
        *version = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        *command = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        for i in 0..(data_size) {
            data[i] = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        let crc_rec_high_byte =
            u16::from(self.pop_front().ok_or(SerialComError::QueueIndexingError)?);
        let crc_rec_low_byte =
            u16::from(self.pop_front().ok_or(SerialComError::QueueIndexingError)?);
        let crc_rec: u16 = (crc_rec_high_byte << 8) & crc_rec_low_byte;
        if crc_num != crc_rec {
            return Err(SerialComError::CRCMismatch);
        }
        Ok(data_size)
    }
}

#[test]
fn test_send() {
    let mut buf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    let ver: u8 = 0x3A;
    let com: u8 = 0x8F;
    let data: [u8; 10] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10];
    buf.send_message(&ver, &com, &data)
        .expect("Couldn't send_message");
    let correctvec: Vec<u8> = [3, 0x3A, 0x8F, 10, 1, 3, 4, 5, 6, 7, 8, 9, 10, 0].to_vec();
    let mut outvec: Vec<u8> = Vec::new();
    for _ in 0..buf.len() {
        outvec.push(buf.pop_front().expect("Element not found"));
    }
    assert_eq!(outvec, correctvec);
}

#[test]
fn test_send_rand() {
    let mut buf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    let mut rng = rand::thread_rng();
    for _trial in 0..1000 {
        let ver: u8 = rand::random::<u8>();
        let com: u8 = rand::random::<u8>();
        let data_size: usize = rng.gen_range(0, 11);
        let mut data: Vec<u8> = Vec::new();
        data.resize(data_size, 0);
        rng.fill(&mut data[..]);
        buf.send_message(&ver, &com, &data)
            .expect("Couldn't send_message");

        let mut correctvec: Vec<u8> = Vec::new();
        correctvec.push(ver);
        correctvec.push(com);
        correctvec.extend(data);
        correctvec
            .cobs_encode()
            .expect("Couldn't encode correct vec");
        let mut outvec: Vec<u8> = Vec::new();
        for _ in 0..buf.len() {
            outvec.push(buf.pop_front().expect("Element not found"));
        }
        assert_eq!(outvec, correctvec);
    }
}

#[test]
fn test_receive() {
    let mut buf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    buf.push_back(2); // 0
    buf.push_back(0xFF);
    buf.push_back(5); // 0
    buf.push_back(0xFF);
    buf.push_back(0xFF);
    buf.push_back(0xFF);
    buf.push_back(0xFF);
    buf.push_back(0);
    let mut ver: u8 = 3;
    let mut com: u8 = 3;
    let mut data: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let n_data = buf
        .receive_message(&mut ver, &mut com, &mut data)
        .expect("Couldn't receive_message");
    assert_eq!(n_data, 4);
    assert_eq!(ver, 0xFF);
    assert_eq!(com, 0);
    for i in 0..4 {
        assert_eq!(data[i], 0xFF);
    }
}

#[test]
fn test_receive_rand() {
    let mut buf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    let mut rng = rand::thread_rng();
    for _trial in 0..1000 {
        let message_size: usize = rng.gen_range(2, 15);
        let mut message: Vec<u8> = Vec::new();
        message.resize(message_size, 0);
        rng.fill(&mut message[..]);
        let corr_ver = message[0];
        let corr_com = message[1];
        let corr_data: Vec<u8> = message[2..].to_vec();
        message
            .cobs_encode()
            .expect("Error while encoding test data!");
        for el in message.iter() {
            buf.push_back(*el);
        }
        let mut ver: u8 = 3;
        let mut com: u8 = 3;
        let mut data: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let n_data = buf
            .receive_message(&mut ver, &mut com, &mut data)
            .expect("Couldn't receive message");
        assert_eq!(n_data, message_size - 2);
        assert_eq!(n_data, corr_data.len());
        assert_eq!(ver, corr_ver);
        assert_eq!(com, corr_com);
        for i in 0..n_data {
            assert_eq!(data[i], corr_data[i]);
        }
        buf.clear();
    }
}
