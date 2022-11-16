#[cfg(feature="use_acid_io")]
use acid_io::{Read, Write, Result};

#[cfg(feature = "use_std")]
use std::io::{Read, Write, Result};






use core::prelude::rust_2021::*;
use alloc::vec;
use alloc::vec::Vec;

use crate::data::*;


/// Implements the Vexrs serial protocol
pub struct VexrsSerial<'a, T: Read + Write> {
    stream: &'a mut T,
    buffer: Vec<u8>,
    pros_compat: bool,
}

impl<'a, T: Read + Write> VexrsSerial<'a, T> {
    /// Creates a new instance of VexrsSerial
    pub fn new(stream: &mut T) -> VexrsSerial<T> {
        VexrsSerial {
            stream,
            buffer: Vec::new(),
            pros_compat: false
        }
    }

    /// Creates a new instance of VexrsSerial with PROS
    /// compatibility enabled
    pub fn new_pros(stream: &mut T) -> VexrsSerial<T> {
        VexrsSerial {
            stream,
            buffer: Vec::new(),
            pros_compat: true
        }
    }

    /// Creates a new serial packet
    pub fn create_serial_packet(pros: bool, data_type: DataType) -> Vec<u8> {

        // Create the packet data
        let packet: Vec<u8> = {
            if pros {
                // Parse for PROS compatibility
                let mut data = Vec::new();
                match data_type {
                    DataType::Print(d) => {
                        data.extend(b"sout");
                        data.extend(d);
                    }, DataType::Error(d) => {
                        data.extend(b"serr");
                        data.extend(d);
                    }, DataType::KernelLog(d) => {
                        if let LogType::Message(d) = d {
                            data.extend(b"kdbg");
                            data.extend(d);
                        }
                    }
                };

                data
            } else {
                // Magic number with the encoded data
                let mut data = vec![0x37u8, 0x31, 0x32, 0x32];
                data.extend(bincode::encode_to_vec(data_type, bincode::config::standard()).unwrap_or_else(|_| Vec::new()));
                data
            }
        };

        // COBS encode the data
        let mut data =  cobs::cobsr::encode_vector(&packet).unwrap_or_else(|_| Vec::<u8>::new());
        data.push(0x00);
        data
    }

    /// Parses a serial packet from an input vector
    pub fn parse_serial_packet(data: Vec<u8>) -> Result<DataType> {
        
        // COBS decode the data
        let parsed_data = cobs::cobsr::decode_vector(&data).unwrap();
        let data = parsed_data;

        // Clear any 0x00 bytes at the beginning
        let data = data.iter().skip_while(|&x| *x == 0x00).cloned().collect::<Vec<u8>>();
        
        // If it starts with sout, serr, or kdbg it is a PROS packet
        if data.starts_with(b"sout") {
            Ok(DataType::Print(data[4..].to_vec()))
        } else if data.starts_with(b"serr") {
            Ok(DataType::Error(data[4..].to_vec()))
        } else if data.starts_with(b"kdbg") {
            Ok(DataType::KernelLog(LogType::Message(data[4..].to_vec())))
        } else if data.starts_with(&[0x37, 0x31, 0x32, 0x32]) { // If it starts with the Vexrs magic number, parse it as such
            // Parse and return the data
            let (decoded, _size): (DataType, usize) = bincode::decode_from_slice(&data[4..], bincode::config::standard()).unwrap_or_else(|_| (DataType::Print(Vec::new()), 0));

            Ok(decoded)
        } else {
            // Otherwise return no data
            Ok(DataType::Print(Vec::new()))
        }
    }

    /// Reads in serial data
    pub fn read_data(&mut self) -> Result<DataType> {
        
        // Read in data so long as there are no 0x00 bytes in the buffer
        while !self.buffer.contains(&0x00) {
            let mut data = [0u8; 0xff];
            
            let size = self.stream.read(&mut data)?;
            
            self.buffer.extend(&data[..size]);
        }
        
        
        // Find the index of the first 0x00 byte and split it off
        let pos = self.buffer.iter().position(|&r| r == 0x00).unwrap();
        let data: Vec<u8> = self.buffer.drain(0..pos).collect();
        
        // If there is still more data on the buffer, pop the last zero
        if !self.buffer.is_empty() && self.buffer[0] == 0x00 {
            self.buffer.drain(0..1).for_each(drop);
        }

        // Parse and return the packet
        VexrsSerial::<T>::parse_serial_packet(data)
    }

    /// Writes serial data
    pub fn write_data(&mut self, data_type: DataType) -> Result<usize> {

        // Create the packet
        let packet = VexrsSerial::<T>::create_serial_packet(self.pros_compat, data_type);

        // Send it
        let size = self.stream.write(&packet)?;

        Ok(size)
    }
}