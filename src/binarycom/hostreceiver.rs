use crate::binarycom::packers;
use crate::binarycom::BinaryCom;
use crate::error::SerialComResult;

use std::sync::mpsc;
use std::thread;

pub struct HostReceiver16 {
    pub rx_thread_handle: thread::JoinHandle<()>,
    rx_reg_read: mpsc::Receiver<(u16, u32)>,
    rx_reg_write: mpsc::Receiver<u16>,
    rx_stream: mpsc::Receiver<(u8, Vec<u8>)>,
}

impl HostReceiver16 {
    pub fn new() -> HostReceiver16 {
        let (mut tx_reg_read, tmp_rx_reg_read) = mpsc::channel();
        let (mut tx_reg_write, tmp_rx_reg_write) = mpsc::channel();
        let (mut tx_stream, tmp_rx_stream) = mpsc::channel();
        let thread_handle = thread::spawn(move || {
            let mut inbuf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
                arraydeque::ArrayDeque::new();
            let mut command: u8 = 0;
            let mut data: [u8; 11] = [0; 11];
            loop {
                match inbuf.receive_message(&mut command, &mut data) {
                    Ok(data_len) => {
                        if let Err(route_error) = message_router(
                            command,
                            &data[0..data_len],
                            &mut tx_reg_read,
                            &mut tx_reg_write,
                            &mut tx_stream,
                        ) {
                            println!(
                                "Error while routing and queuing message from device to host: {}",
                                route_error
                            );
                        }
                    }
                    Err(recv_error) => {
                        println!("Error while receiving dev -> host message: {}", recv_error)
                    }
                }
            }
        });
        HostReceiver16 {
            rx_thread_handle: thread_handle,
            rx_reg_read: tmp_rx_reg_read,
            rx_reg_write: tmp_rx_reg_write,
            rx_stream: tmp_rx_stream,
        }
    }
    pub fn get_rxs(
        &self,
    ) -> (
        &mpsc::Receiver<(u16, u32)>,
        &mpsc::Receiver<u16>,
        &mpsc::Receiver<(u8, Vec<u8>)>,
    ) {
        (&self.rx_reg_read, &self.rx_reg_write, &self.rx_stream)
    }
}

fn message_router(
    command: u8,
    data: &[u8],
    tx_reg_read: &mut mpsc::Sender<(u16, u32)>,
    tx_reg_write: &mut mpsc::Sender<u16>,
    tx_stream: &mut mpsc::Sender<(u8, Vec<u8>)>,
) -> SerialComResult<()> {
    match command {
        0u8 => {
            println!("Error: unexpected command received: 0x{:02X}", command);
        }
        1u8 => {
            // read register
            let (reg_num, reg_val) = packers::host_read_reg_unpack(data)?;
            tx_reg_read.send((reg_num, reg_val))?;
        }
        2u8 => {
            // write register
            let reg_num = packers::host_write_reg_unpack(data)?;
            tx_reg_write.send(reg_num)?;
        }
        0x3u8..=0x7Fu8 => {
            println!("Error: unexpected command received: 0x{:02X}", command);
        }
        0x80u8..=0xFFu8 => {
            tx_stream.send((command, data.to_vec()))?;
        }
    }
    Ok(())
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
pub fn unpack_tx(command: u8, data: Vec<u8>) -> SerialComResult<Vec<u32>> {
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
