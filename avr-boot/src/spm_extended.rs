use crate::{spm_normal, DataPage};

pub type Address = u32;

pub fn store_page(address: Address, data: &DataPage) {
    fill_page_buffer(address, data);

    erase_page(address);
    busy_wait();
    write_page(address);
    busy_wait();
    rww_enable();
}

pub fn erase_page(address: Address) {
    rampz((address >> 16) as u8);
    spm_normal::erase_page(address as u16);
}

pub fn fill_page(address: Address, data: u16) {
    spm_normal::fill_page(address as u16, data);
}

pub fn write_page(address: Address) {
    rampz((address >> 16) as u8);
    spm_normal::write_page(address as u16);
}

pub fn fill_page_buffer(address: Address, data: &DataPage) {
    rampz((address >> 16) as u8);
    spm_normal::fill_page_buffer(address as u16, data);
}

pub fn lock_bits_set(lock_bits: u8) {
    rampz(0);
    spm_normal::lock_bits_set(lock_bits);
}

pub fn rww_enable() {
    spm_normal::rww_enable();
}

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
