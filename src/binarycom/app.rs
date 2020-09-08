use crate::binarycom::hostreceiver::HostReceiver16;
use crate::binarycom::packers;
use crate::binarycom::BinaryCom;
use crate::error::SerialComResult;

use std::convert::TryFrom;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum RegisterBitWidth {
    Eight,
    ThirtyTwo,
}

pub struct BinaryComApp<T: 'static + Write> {
    pub stream_thread_handle: thread::JoinHandle<()>,
    serial_outfile: T,
    hostreceiver: HostReceiver16,
    outbuf: arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping>,
    regbitwidth: RegisterBitWidth,
}

impl<T: 'static + Write> BinaryComApp<T> {
    pub fn new<U, V>(
        register_bit_width: RegisterBitWidth,
        mut serial_infile: U,
        mut serial_outfile: T,
        mut stream_outfile: V,
    ) -> BinaryComApp<T>
    where
        U: 'static + Read + Send,
        V: 'static + Write + Send,
    {
        let (hr, rx_stream) = HostReceiver16::new(serial_infile);
        let stream_thread = thread::spawn(move || loop {
            match rx_stream.recv() {
                Ok((command, data_vec)) => match packers::unpack_stream(command, data_vec) {
                    Ok(data) => {
                        for datum in data {
                            if let Err(write_err) = write!(stream_outfile, "{},", datum) {
                                eprintln!(
                                    "Error while writing stream data to the file: {}",
                                    write_err
                                );
                            }
                        }
                        if let Err(write_err) = write!(stream_outfile, "\n") {
                            eprintln!("Error while writing stream data to the file: {}", write_err);
                        }
                    }
                    Err(unpack_err) => {
                        eprintln!("Error while unpacking stream data: {}", unpack_err)
                    }
                },
                Err(mpsc::RecvError) => {
                    eprintln!("rx_stream disconnected, closing stream thread");
                    return;
                }
            }
        });
        BinaryComApp {
            stream_thread_handle: stream_thread,
            serial_outfile: serial_outfile,
            hostreceiver: hr,
            outbuf: arraydeque::ArrayDeque::new(),
            regbitwidth: register_bit_width,
        }
    }
    pub fn write_reg(&mut self, reg_num: u16, reg_val: u32) -> SerialComResult<()> {
        match self.regbitwidth {
            RegisterBitWidth::Eight => self
                .outbuf
                .host_write_reg8(reg_num, u8::try_from(reg_val)?)?,
            RegisterBitWidth::ThirtyTwo => self.outbuf.host_write_reg32(reg_num, reg_val)?,
        }
        self.outbuf.write_to_serial(self.serial_outfile)?;
        loop {
            match self
                .hostreceiver
                .rx_reg_write
                .recv_timeout(Duration::from_millis(200))
            {
                Ok(reg_num_rec) => {
                    if reg_num_rec == reg_num {
                        return Ok(());
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    panic!("write_reg rx_reg_write timeout while waiting for response")
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    panic!("write_reg rx_reg_write disconnected while waiting for response")
                }
            }
        }
    }
    pub fn read_reg(&mut self, reg_num: u16) -> SerialComResult<u32> {
        self.outbuf.host_read_reg(reg_num)?;
        self.outbuf.write_to_serial(self.serial_outfile)?;
        loop {
            match self
                .hostreceiver
                .rx_reg_read
                .recv_timeout(Duration::from_millis(200))
            {
                Ok((reg_num_rec, reg_val_rec)) => {
                    if reg_num_rec == reg_num {
                        return Ok(reg_val_rec);
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    panic!("read_reg rx_reg_read timeout while waiting for response")
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    panic!("read_reg rx_reg_read disconnected while waiting for response")
                }
            }
        }
    }
}
