use rand::prelude::*;
use std::convert::TryFrom;

use crate::error::SerialComError;
use crate::error::SerialComResult;

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
