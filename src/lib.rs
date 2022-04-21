#![no_std]

// Externs for various crates we use
extern crate alloc;
extern crate core;
extern crate vexv5rt;
extern crate newlib_alloc;

// Modules that we define
mod internal; // Internal functions that the user should not use
pub mod serial; // Actual serial implementation