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
