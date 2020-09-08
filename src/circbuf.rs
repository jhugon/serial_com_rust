use crate::error::SerialComError;
use crate::error::SerialComResult;

use rand::prelude::*;
use std::convert::TryFrom;
use std::io::Read;
use std::io::Write;

extern crate arraydeque;

/// Trait for circular buffer helper functions useful for serial communications
///
/// Implemented for:
///
///  arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping>
///  arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping>
///
pub trait CircBufExt {
    /// Pretty print circular buffer
    fn print(&self);
    /// Push back n random elements
    ///
    /// perc_extra_zero is the percentage of elements that will be 0, in addition to the number that
    /// would be 0 from the uniform distribution.
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32);
    /// Removes the first n elements off the front of the buffer
    ///
    /// Will return Ok(new length) on success or Err(QueueIndexingError) if not enough elements in
    /// buffer to remove n.
    fn remove_front_n(&mut self, n: &usize) -> SerialComResult<usize>;
    //    fn write_to_file<T>(&mut self, outfile: T) -> SerialComResult<()>
    //    where
    //        T: Write;
    //    fn read_from_file<T>(&mut self, infile: T) -> SerialComResult<()>
    //    where
    //        T: Read;
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 8], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
    fn remove_front_n(&mut self, n: &usize) -> SerialComResult<usize> {
        for _ in 0..*n {
            self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        Ok(self.len())
    }
    //    fn write_to_file<T>(&mut self, outfile: T) -> SerialComResult<()>
    //    where
    //        T: Write,
    //    {
    //        let slices_tuple = self.as_slices();
    //        let slices = [slices_tuple.0, slices_tuple.1];
    //        for slice in slices.iter() {
    //            loop {
    //                match outfile.write_all(slice) {
    //                    Ok(_) => break,
    //                    Err(std::io::ErrorKind::Interrupted) => {}
    //                    Err(write_error) => {
    //                        return Err(SerialComError::FileWrite(write_error));
    //                    }
    //                }
    //            }
    //        }
    //        Ok(())
    //    }
    //    fn read_from_file<T>(&mut self, infile: T) -> SerialComResult<()>
    //    where
    //        T: Read,
    //    {
    //        let mut buf: Vec<u8> = Vec::with_capacity(4);
    //        match infile.read(buf) {
    //            Ok(nread) => {
    //                self.extend_back(buf.iter());
    //                Ok(())
    //            }
    //            Err(std::io::ErrorKind::Interrupted) => Ok(()),
    //            Err(read_error) => SerialComError::FileRead(read_error),
    //        }
    //    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 16], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
    fn remove_front_n(&mut self, n: &usize) -> SerialComResult<usize> {
        for _ in 0..*n {
            self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        Ok(self.len())
    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 32], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
    fn remove_front_n(&mut self, n: &usize) -> SerialComResult<usize> {
        for _ in 0..*n {
            self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        Ok(self.len())
    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 64], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }

    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
    fn remove_front_n(&mut self, n: &usize) -> SerialComResult<usize> {
        for _ in 0..*n {
            self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        Ok(self.len())
    }
}

impl CircBufExt for arraydeque::ArrayDeque<[u8; 128], arraydeque::Wrapping> {
    fn print(&self) {
        println!(
            "ArrayDeque: capacity: {}, len: {}",
            self.capacity(),
            self.len()
        );
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push_back(0u8);
            } else {
                self.push_back(rng.gen::<u8>());
            }
        }
    }
    fn remove_front_n(&mut self, n: &usize) -> SerialComResult<usize> {
        for _ in 0..*n {
            self.pop_front().ok_or(SerialComError::QueueIndexingError)?;
        }
        Ok(self.len())
    }
}

impl CircBufExt for Vec<u8> {
    fn print(&self) {
        println!("Vector: capacity: {}, len: {}", self.capacity(), self.len());
        print!("  [");
        let lasti = self.len() - 1;
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{}]", element)
            } else {
                print!("{}, ", element)
            }
        }
        print!("  [");
        for (i, element) in self.iter().enumerate() {
            if i == lasti {
                println!("{:02X}]", element)
            } else {
                print!("{:02X}, ", element)
            }
        }
        print!("  ");
        for element in self.iter() {
            let ch_result = char::try_from(*element);
            match ch_result {
                Ok(ch) => print!("{}", ch.escape_default()),
                Err(_) => print!("{}", '\u{FFFD}'),
            }
        }
        println!("");
    }
    fn push_back_rand(&mut self, n: &usize, perc_extra_zero: &u32) {
        let mut rng = thread_rng();
        for _i_element in 0..*n {
            if rng.gen_ratio(*perc_extra_zero, 100) {
                self.push(0u8);
            } else {
                self.push(rng.gen::<u8>());
            }
        }
    }
    fn remove_front_n(&mut self, n: &usize) -> SerialComResult<usize> {
        for _ in 0..*n {
            self.remove(0);
        }
        Ok(self.len())
    }
}
