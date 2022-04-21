
use alloc::vec::Vec;


/// Sends raw data over the serial channel
/// This is only marked as unsafe because it has no checks and should
/// not be used by anything other than a wrapper struct.
pub unsafe fn send_serial_raw(mut data: Vec<u8>) {
    vexv5rt::vexSerialWriteBuffer(1, data.as_mut_ptr(), data.len() as u32);
}



