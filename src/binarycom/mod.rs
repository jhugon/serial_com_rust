pub mod app;
pub mod hostreceiver;
pub mod packers;

//use crate::circbuf::CircBufExt;
#[cfg(test)]
use crate::circbuf::CircBufExt;
use crate::cobs::COBSExt;
use crate::crc::CRCExt;
use crate::error::{SerialComError, SerialComResult};

#[cfg(test)]
use rand::prelude::*;

/// Meant to be used as methods on arraydeque::ArrayDeque<[u8; N], arraydeque::Wrapping>
pub trait BinaryCom {
    /// Put a message in output buffer
    ///
    /// Do appropriate formatting, byte stuffing, checksum, etc.
    ///
    /// The max data buffer size is the max message size - 5. That is 1 byte overhead, 1 byte comma,
    /// 1 byte command, 2 bytes checksum.
    ///
    /// Returns final message length
    fn send_message(&mut self, command: &u8, data: &[u8]) -> SerialComResult<usize>;

    /// Read message (if one exists) from input buffer into message
    ///
    /// Do appropriate formatting, byte stuffing, checksum, etc.
    ///
    /// The data buffer must be the max message size - 5. That is 1 byte overhead, 1 byte comma,
    /// 1 byte command, 2 bytes checksum.
    ///
    /// Returns the received data length
    fn receive_message(&mut self, command: &mut u8, data: &mut [u8]) -> SerialComResult<usize>;

    /// Initiate register read
    ///
    /// Meant to be used on host to read a device register
    fn host_read_reg(&mut self, reg_num: u16) -> SerialComResult<u32> {
        let command = 1u8;
        let mut data: [u8; 2] = [0; 2];
        packers::host_read_reg_pack(reg_num, &mut data)?;
        self.send_message(&command, &data)?;
        Ok(0u32)
    }

    /// Initiate 8-bit wide register write
    ///
    /// Meant to be used on host to write a device register
    ///
    fn host_write_reg8(&mut self, reg_num: u16, reg_val: u8) -> SerialComResult<()> {
        let command = 2u8;
        let mut data: [u8; 3] = [0; 3];
        packers::host_write_reg8_pack(reg_num, reg_val, &mut data)?;
        self.send_message(&command, &data)?;
        Ok(())
    }

    /// Initiate 32-bit wide register write
    ///
    /// Meant to be used on host to write a device register
    fn host_write_reg32(&mut self, reg_num: u16, reg_val: u32) -> SerialComResult<()> {
        let command = 2u8;
        let mut data: [u8; 6] = [0; 6];
        packers::host_write_reg32_pack(reg_num, reg_val, &mut data)?;
        self.send_message(&command, &data)?;
        Ok(())
    }
}

impl BinaryCom for arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> {
    fn send_message(&mut self, command: &u8, data: &[u8]) -> SerialComResult<usize> {
        if data.len() > 16 - 5 {
            return Err(SerialComError::SliceTooBig);
        }
        self.clear();
        self.push_back(*command);
        for el in data {
            self.push_back(*el);
        }
        let (crc_high_byte, crc_low_byte) = self.compute_crc_bytes(self.len())?;
        self.push_back(crc_high_byte);
        self.push_back(crc_low_byte);
        self.cobs_encode()?;
        Ok(self.len())
    }
    fn receive_message(&mut self, command: &mut u8, data: &mut [u8]) -> SerialComResult<usize> {
        if data.len() < 16 - 5 {
            println!("receive_message: data len: {}", data.len());
            return Err(SerialComError::SliceTooSmall);
        }
        let msg_chk_size = self.cobs_decode()?;
        let msg_size = msg_chk_size - 2;
        let data_size = msg_size - 1;
        let (crc_high_byte, crc_low_byte) = self.compute_crc_bytes(msg_size)?;
        *command = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        for i in 0..(data_size) {
            data[i] = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        let crc_rec_high_byte = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        let crc_rec_low_byte = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        if crc_high_byte != crc_rec_high_byte {
            #[cfg(test)]
            {
                println!(
                    "Error CRCMismatch: received: {:02X}{:02X}, computed: {:02X}{:02X}",
                    crc_rec_high_byte, crc_rec_low_byte, crc_high_byte, crc_low_byte
                );
                println!(
                    "Error for message command: {:02X} data size: {}",
                    command, data_size
                )
            }
            return Err(SerialComError::CRCMismatch);
        }
        if crc_low_byte != crc_rec_low_byte {
            #[cfg(test)]
            {
                println!(
                    "Error CRCMismatch: received: {:02X}{:02X}, computed: {:02X}{:02X}",
                    crc_rec_high_byte, crc_rec_low_byte, crc_high_byte, crc_low_byte
                );
                println!(
                    "Error for message command: {:02X} data size: {}",
                    command, data_size
                )
            }
            return Err(SerialComError::CRCMismatch);
        }
        Ok(data_size)
    }
}

impl BinaryCom for arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> {
    fn send_message(&mut self, command: &u8, data: &[u8]) -> SerialComResult<usize> {
        if data.len() > 64 - 5 {
            return Err(SerialComError::SliceTooBig);
        }
        self.clear();
        self.push_back(*command);
        for el in data {
            self.push_back(*el);
        }
        let (crc_high_byte, crc_low_byte) = self.compute_crc_bytes(self.len())?;
        self.push_back(crc_high_byte);
        self.push_back(crc_low_byte);
        self.cobs_encode()?;
        Ok(self.len())
    }
    fn receive_message(&mut self, command: &mut u8, data: &mut [u8]) -> SerialComResult<usize> {
        if data.len() < 64 - 5 {
            return Err(SerialComError::SliceTooSmall);
        }
        let msg_chk_size = self.cobs_decode()?;
        let msg_size = msg_chk_size - 2;
        let data_size = msg_size - 1;
        let (crc_high_byte, crc_low_byte) = self.compute_crc_bytes(msg_size)?;
        *command = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        for i in 0..(data_size) {
            data[i] = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        let crc_rec_high_byte = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        let crc_rec_low_byte = self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        if crc_high_byte != crc_rec_high_byte {
            return Err(SerialComError::CRCMismatch);
        }
        if crc_low_byte != crc_rec_low_byte {
            return Err(SerialComError::CRCMismatch);
        }
        Ok(data_size)
    }
}

#[test]
fn test_send() {
    let mut buf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    let com: u8 = 0x8F;
    let data: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    buf.send_message(&com, &data)
        .expect("Couldn't send_message");
    let correctvec: Vec<u8> = [2, 0x8F, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 68, 78, 0].to_vec();
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
        let com: u8 = rand::random::<u8>();
        let data_size: usize = rng.gen_range(0, 12);
        let mut data: Vec<u8> = Vec::new();
        data.resize(data_size, 0);
        rng.fill(&mut data[..]);
        buf.send_message(&com, &data)
            .expect("Couldn't send_message");

        let mut correctvec: Vec<u8> = Vec::new();
        correctvec.push(com);
        correctvec.extend(data);
        let (crc_h, crc_l) = correctvec
            .compute_crc_bytes(correctvec.len())
            .expect("Couldn't compute CRC");
        correctvec.push(crc_h);
        correctvec.push(crc_l);
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
    buf.push_back(8);
    buf.push_back(0);
    buf.push_back(0);
    buf.push_back(0);
    buf.push_back(0);
    buf.push_back(0);
    buf.push_back(0);
    let (crc_h, crc_l) = buf
        .compute_crc_bytes(buf.len())
        .expect("Couldn't compute CRC");
    buf.push_back(crc_h);
    buf.push_back(crc_l);
    buf.cobs_encode().expect("Couldn't encode buf");
    buf.print();
    let mut com: u8 = 3;
    let mut data: [u8; 11] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let n_data = buf
        .receive_message(&mut com, &mut data)
        .expect("Couldn't receive_message");
    assert_eq!(n_data, 6);
    assert_eq!(com, 8);
    for i in 0..6 {
        assert_eq!(data[i], 0);
    }
}

#[test]
fn test_receive_rand() {
    let mut buf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    let mut rng = rand::thread_rng();
    for _trial in 0..1000 {
        let message_size: usize = rng.gen_range(1, 13);
        let mut message: Vec<u8> = Vec::new();
        message.resize(message_size, 0);
        rng.fill(&mut message[..]);
        let corr_com = message[0];
        let corr_data: Vec<u8> = message[1..].to_vec();
        let (crc_h, crc_l) = message
            .compute_crc_bytes(message.len())
            .expect("Couldn't compute CRC");
        message.push(crc_h);
        message.push(crc_l);
        message
            .cobs_encode()
            .expect("Error while encoding test data!");
        for el in message.iter() {
            buf.push_back(*el);
        }
        message.print();
        buf.print();
        let mut com: u8 = 3;
        let mut data: [u8; 11] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let n_data = buf
            .receive_message(&mut com, &mut data)
            .expect("Couldn't receive message");

        assert_eq!(n_data, message_size - 1);
        assert_eq!(n_data, corr_data.len());
        assert_eq!(com, corr_com);
        for i in 0..n_data {
            assert_eq!(data[i], corr_data[i]);
        }
        buf.clear();
    }
}

#[test]
fn test_receive_nothing() {
    let mut buf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
        arraydeque::ArrayDeque::new();
    let mut com: u8 = 3;
    let mut data: [u8; 11] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    buf.receive_message(&mut com, &mut data)
        .expect_err("Should be COBSTooLittleData error!");
}
