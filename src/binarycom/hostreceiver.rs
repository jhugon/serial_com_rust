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
