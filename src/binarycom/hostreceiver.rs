use crate::binarycom::packers;
use crate::binarycom::BinaryCom;
use crate::error::SerialComResult;

use std::io::Read;
use std::sync::mpsc;
use std::thread;

pub struct HostReceiver16 {
    pub rx_thread_handle: thread::JoinHandle<()>,
    pub rx_reg_read: mpsc::Receiver<(u16, u32)>,
    pub rx_reg_write: mpsc::Receiver<u16>,
}

impl HostReceiver16 {
    /// returns both a HostReceiver16 and rx_stream: the receiver for streaming messages
    pub fn new<T>(mut serial_infile: T) -> (HostReceiver16, mpsc::Receiver<(u8, Vec<u8>)>)
    where
        T: 'static + Read + Send,
    {
        let (mut tx_reg_read, tmp_rx_reg_read) = mpsc::channel();
        let (mut tx_reg_write, tmp_rx_reg_write) = mpsc::channel();
        let (mut tx_stream, rx_stream) = mpsc::channel();
        let thread_handle = thread::spawn(move || {
            let mut inbuf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> =
                arraydeque::ArrayDeque::new();
            let mut command: u8 = 0;
            let mut data: [u8; 11] = [0; 11];
            loop {
                if let Err(read_serial_err) = inbuf.read_from_serial(serial_infile) {
                    eprintln!("Error while reading from serial: {}", read_serial_err);
                    return;
                }
                match inbuf.receive_message(&mut command, &mut data) {
                    Ok(data_len) => {
                        if let Err(route_error) = message_router(
                            command,
                            &data[0..data_len],
                            &mut tx_reg_read,
                            &mut tx_reg_write,
                            &mut tx_stream,
                        ) {
                            eprintln!(
                                "Error while routing and queuing message from device to host: {}",
                                route_error
                            );
                        }
                    }
                    Err(recv_error) => {
                        eprintln!("Error while receiving dev -> host message: {}", recv_error)
                    }
                }
            }
        });
        (
            HostReceiver16 {
                rx_thread_handle: thread_handle,
                rx_reg_read: tmp_rx_reg_read,
                rx_reg_write: tmp_rx_reg_write,
            },
            rx_stream,
        )
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
            eprintln!("Error: unexpected command received: 0x{:02X}", command);
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
            eprintln!("Error: unexpected command received: 0x{:02X}", command);
        }
        0x80u8..=0xFFu8 => {
            tx_stream.send((command, data.to_vec()))?;
        }
    }
    Ok(())
}
