//! Low level API for MCUs with >64k of storage.
//! use [crate::spm] to get the correct mode for your target MCU

use crate::{spm_normal, Address, DataPage};

// pub type Address = u32;

/// Store a whole page into program memory by erasing the page, filling the buffer,
/// and writing the buffer to the program memory.  
/// `address` must be page aligned.
pub fn store_page(address: Address, data: &DataPage) {
    fill_page_buffer(address, data);

    erase_page(address);
    busy_wait();
    write_page(address);
    busy_wait();
    rww_enable();
}

/// Erase the page from program memory
///
/// The PCPAGE part of the address is used to address the page, the PCWORD part must be zero
pub fn erase_page(address: Address) {
    rampz(address.ramp());
    spm_normal::erase_page(address);
}

/// Write data to the page buffer
///
/// Only the PCWORD part of the address actually matters, the size of which varies according to SPM_PAGESIZE_BYTES
pub fn fill_page(address: Address, data: u16) {
    spm_normal::fill_page(address, data);
}

/// Write the page from the buffer to the program memory
///
/// The PCPAGE part of the address is used to address the page, the PCWORD part must be zero
pub fn write_page(address: Address) {
    rampz(address.ramp());
    spm_normal::write_page(address);
}

/// Fill the whole buffer at once
///
/// If have the data in a RAM buffer already, this is slightly smaller
/// and faster than using [`fill_page`] in a loop
pub fn fill_page_buffer(address: Address, data: &DataPage) {
    rampz(address.ramp());
    spm_normal::fill_page_buffer(address, data);
}

pub fn lock_bits_set(lock_bits: u8) {
    rampz(0);
    spm_normal::lock_bits_set(lock_bits);
}

/// Re-enable the RWW section after programming, to enable it to be read
pub fn rww_enable() {
    spm_normal::rww_enable();
}

/// Wait for the current SPM operation to complete.
///
/// On devices with a RWW section, the CPU is not halted during the SPM operation if the RWW section is being written to.
/// Therefore it is important that we make sure the operation is complete before trying to do the next operation.
pub fn busy_wait() {
    spm_normal::busy_wait()
}

#[cfg(extended_addressing)]
fn rampz(value: u8) {
    unsafe {
        core::ptr::write_volatile(crate::RAMPZ, value);
    }
}

#[cfg(not(extended_addressing))]
fn rampz(_value: u8) {
    // Do nothing in normal mode - RAMPZ does not exist
}
