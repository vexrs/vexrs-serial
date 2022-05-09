#![cfg_attr(not(feature="use_std"), no_std)]


// Externs for various crates we use
extern crate alloc;
extern crate core;


pub mod protocol; // Contains the basic protocol implementation

pub mod data; // Various data classes that are used in the protocol