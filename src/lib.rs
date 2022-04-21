#![cfg_attr(not(feature="use_std"), no_std)]


// Externs for various crates we use
extern crate alloc;
extern crate core;

// Modules that we define
#[cfg(feature = "v5")]
pub mod internal; // Internal functions that the user should not use
#[cfg(feature = "v5")]
pub mod serial; // Actual serial implementation

pub mod protocol; // Contains the basic protocol implementation