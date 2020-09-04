use crate::error::{SerialComError, SerialComResult};

use std::convert::TryFrom;

/// Unpack register read message
///
/// Returns result holding register number
pub fn dev_read_reg_unpack(data: &[u8]) -> SerialComResult<u16> {
    if data.len() < 2 {
        return Err(SerialComError::SliceTooSmall);
    }
    let reg_num = (u16::from(data[0]) << (8 * 1) & 0xFF) & (u16::from(data[1]) << (8 * 0) & 0xFF);
    Ok(reg_num)
}

/// Unpack 8-bit register write message
///
/// Returns result holding (register number, register value)
pub fn dev_write_reg8_unpack(data: &[u8]) -> SerialComResult<(u16, u8)> {
    if data.len() < 3 {
        return Err(SerialComError::SliceTooSmall);
    }
    let reg_num = (u16::from(data[0]) << (8 * 1) & 0xFF) & (u16::from(data[1]) << (8 * 0) & 0xFF);
    let reg_val = data[2];
    Ok((reg_num, reg_val))
}
/// Unpack 8-bit register write message
///
/// Returns result holding (register number, register value)
pub fn dev_write_reg32_unpack(data: &[u8]) -> SerialComResult<(u16, u32)> {
    if data.len() < 6 {
        return Err(SerialComError::SliceTooSmall);
    }
    let reg_num: u16 =
        (u16::from(data[0]) << (8 * 1) & 0xFF) & (u16::from(data[1]) << (8 * 0) & 0xFF);
    let reg_val: u32 = (u32::from(data[2]) << (8 * 3) & 0xFF)
        & (u32::from(data[3]) << (8 * 2) & 0xFF)
        & (u32::from(data[4]) << (8 * 1) & 0xFF)
        & (u32::from(data[5]) << (8 * 0) & 0xFF);
    Ok((reg_num, reg_val))
}

/// Respond to 8-bit register read message
///
/// packs data portion of message
///
/// returns Result with length of data
pub fn dev_read_reg8_pack(reg_num: u16, reg_val: u8, data: &mut [u8]) -> SerialComResult<u8> {
    if data.len() < 3 {
        return Err(SerialComError::SliceTooSmall);
    }
    data[0] = u8::try_from(reg_num >> 8 & 0xFF)?;
    data[1] = u8::try_from(reg_num & 0xFF)?;
    data[2] = reg_val;
    Ok(3u8)
}

/// Respond to 32-bit register read message
///
/// packs data portion of message
///
/// returns Result with length of data
pub fn dev_read_reg32_pack(reg_num: u16, reg_val: u32, data: &mut [u8]) -> SerialComResult<u8> {
    if data.len() < 6 {
        return Err(SerialComError::SliceTooSmall);
    }
    data[0] = u8::try_from(reg_num >> 8 & 0xFF)?;
    data[1] = u8::try_from(reg_num & 0xFF)?;
    data[2] = u8::try_from(reg_val >> (8 * 3) & 0xFF)?;
    data[3] = u8::try_from(reg_val >> (8 * 2) & 0xFF)?;
    data[4] = u8::try_from(reg_val >> (8 * 1) & 0xFF)?;
    data[5] = u8::try_from(reg_val >> (8 * 0) & 0xFF)?;
    Ok(6u8)
}

/// Respond to 8-bit register write message
///
/// packs data portion of message
///
/// returns Result with length of data
pub fn dev_write_reg_pack(reg_num: u16, data: &mut [u8]) -> SerialComResult<u8> {
    if data.len() < 2 {
        return Err(SerialComError::SliceTooSmall);
    }
    data[0] = u8::try_from(reg_num >> 8 & 0xFF)?;
    data[1] = u8::try_from(reg_num & 0xFF)?;
    Ok(2u8)
}

/// Pack a read message
///
/// returns Result with length of data
pub fn host_read_reg_pack(reg_num: u16, data: &mut [u8]) -> SerialComResult<u8> {
    dev_write_reg_pack(reg_num, data)
}

/// Pack a write message to a 8 bit register
///
/// returns Result with length of data
pub fn host_write_reg8_pack(reg_num: u16, reg_val: u8, data: &mut [u8]) -> SerialComResult<u8> {
    dev_read_reg8_pack(reg_num, reg_val, data)
}

/// Pack a write message to a 32 bit register
///
/// returns Result with length of data
pub fn host_write_reg32_pack(reg_num: u16, reg_val: u32, data: &mut [u8]) -> SerialComResult<u8> {
    dev_read_reg32_pack(reg_num, reg_val, data)
}

pub fn host_write_reg_unpack(data: &[u8]) -> SerialComResult<u16> {
    dev_read_reg_unpack(data)
}

pub fn host_read_reg_unpack(data: &[u8]) -> SerialComResult<(u16, u32)> {
    match data.len() {
        3 => {
            let (reg_num, reg_val) = dev_write_reg8_unpack(data)?;
            Ok((reg_num, u32::from(reg_val)))
        }
        6 => dev_write_reg32_unpack(data),
        _ => Err(SerialComError::SliceTooSmall),
    }
}

/// unpack tx messages
///
/// command unpacking:
///
/// Lowest 3 bits are the word size in bits / 4
/// Next 3 bits are the number of words in a single sample (for simultaneous measurements)
/// The top 2 bits are reserved and should be 0
///
/// command = 0x80 means the data is UTF-8 text
pub fn unpack_stream(command: u8, data: Vec<u8>) -> SerialComResult<Vec<u32>> {
    let word_size_bits = (command & 0b111) * 4;
    let n_per_sample_word = command >> 3 & 0x1F;
    if n_per_sample_word != 1 {
        unimplemented!("Haven't implemented multiple words per sample");
    }
    match word_size_bits {
        4 => Ok(data
            .iter()
            .map(|x| vec![x >> 0xF, x & 0xF])
            .flatten()
            .map(|x| u32::from(x))
            .collect()),
        8 => Ok(data.iter().map(|x| u32::from(*x)).collect()),
        12 => {
            if data.len() % 3 != 0 {
                panic!("data for 12 bit ints must be a multiple of 3 long!");
            }
            let mut result: Vec<u32> = Vec::new();
            for i in (0..data.len()).step_by(3) {
                let el1 = u32::from(data[i]) << 4 & (u32::from(data[i + 1]) >> 4);
                result.push(el1);
                let el2 = (u32::from(data[i + 1]) & 0xF) << 4 & u32::from(data[i + 2]);
                result.push(el2);
            }
            Ok(result)
        }
        16 => {
            if data.len() % 2 != 0 {
                panic!("data for 16 bit ints must be a multiple of 2 long!");
            }
            let datau32s = data.iter().map(|x| u32::from(*x));
            let evens = datau32s.clone().step_by(2);
            let odds = datau32s.skip(1).step_by(2);
            Ok(evens.zip(odds).map(|(e, o)| e << 8 & o).collect()) // msb first--big-endian
        }
        32 => {
            if data.len() % 4 != 0 {
                panic!("data for 32 bit ints must be a multiple of 4 long!");
            }
            let datau32s = data.iter().map(|x| u32::from(*x));
            let d0s = datau32s.clone().step_by(4);
            let d1s = datau32s.clone().skip(1).step_by(4);
            let d2s = datau32s.clone().skip(2).step_by(4);
            let d3s = datau32s.clone().skip(3).step_by(4);
            // msb first--big-endian
            let d01s = d0s.zip(d1s).map(|(d0, d1)| d0 << 8 & d1);
            let d23s = d2s.zip(d3s).map(|(d0, d1)| d0 << 8 & d1);
            let d0123s = d01s.zip(d23s).map(|(d01, d12)| d01 << 16 & d12);
            Ok(d0123s.collect())
        }
        _ => unimplemented!("Only implemented 8, 16, 32 bit streaming!"),
    }
}
