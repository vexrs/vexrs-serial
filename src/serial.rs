



use core::prelude::rust_2021::*;
use alloc::vec::Vec;
use acid_io::{Read, Write};

/// Basic serial Read/Write implementation
/// with buffering
#[derive(Default)]
pub struct Serial {
    buffer: Vec<u8>,
}

impl Serial {
    pub fn new() -> Serial {
        Serial { buffer: Vec::new() }
    }
}


impl Read for Serial {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, acid_io::Error> {
        // Read until we have enough bytes
        while self.buffer.len() < buf.len() {

            // Read in the data
            let data = unsafe { vexv5rt::vexSerialReadChar(1) };

            // If it is out of bounds exit the loop
            if !(0..=0xff).contains(&data) {
                break;
            }

            self.buffer.push(data as u8);
        }
        
        // Figure how many bytes we have to copy over
        let len = core::cmp::min(buf.len(), self.buffer.len());

        // Drain len bytes from the buffer
        let mut data: Vec<u8> = self.buffer.drain(0..len).collect();
        
        // Make sure the size is buf.len()
        data.resize(buf.len(), 0u8);

        // Copy into buf
        buf.copy_from_slice(&data);

        Ok(len)
    }
}


impl Write for Serial {
    fn write(&mut self, buf: &[u8]) -> Result<usize, acid_io::Error> {

        unsafe {
            crate::internal::send_serial_raw(buf.to_vec());
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), acid_io::Error> {

        

        Ok(())
    }
}