#![no_std]
#![feature(asm_experimental_arch)]
#![doc = include_str!("../../README.md")]

mod address;
mod buffer;

use core::ops::Deref;
use const_env__value::value_from_env;
pub mod spm;
pub use address::Address;
pub use buffer::PageBuffer;

/// Total size of the SPM page buffer, for the current MCU target
pub const SPM_PAGESIZE_BYTES: usize = value_from_env!("AVR_BOOT_SPM_PAGESIZE": usize);

/// Total length in 16 byte words of the SPM page buffer, for the current MCU target
pub const SPM_PAGESIZE_WORDS: usize = SPM_PAGESIZE_BYTES / 2;

#[cfg(extended_addressing)]
#[doc(hidden)]
pub const RAMPZ: *mut u8 = value_from_env!("AVR_RAMPZ": u8) as *mut u8;
#[cfg(target_arch = "avr")]
const SPMCSR: *mut u8 = value_from_env!("AVR_BOOT_SPMCSR": u8) as *mut u8;
#[cfg(target_arch = "avr")]
const SPMCSR_ADDR: u16 = value_from_env!("AVR_BOOT_SPMCSR": u8) as u16;
#[cfg(target_arch = "avr")]
const PAGE_ERASE: u8 = value_from_env!("AVR_BOOT_PAGE_ERASE": u8);
#[cfg(target_arch = "avr")]
const PAGE_WRITE: u8 = value_from_env!("AVR_BOOT_PAGE_WRITE": u8);
#[cfg(target_arch = "avr")]
const PAGE_FILL: u8 = value_from_env!("AVR_BOOT_PAGE_FILL": u8);
#[cfg(target_arch = "avr")]
const LOCK_BITS_SET: u8 = value_from_env!("AVR_BOOT_LOCK_BITS_SET": u8);
#[cfg(all(target_arch = "avr", rww_enable))]
const RWW_ENABLE: u8 = value_from_env!("AVR_BOOT_RWW_ENABLE": u8);

/// NewType, an array of memory the same size as the page buffer
pub struct DataPage(pub [u16; SPM_PAGESIZE_WORDS]);

impl<'a> From<&'a [u16; SPM_PAGESIZE_WORDS]> for &'a DataPage {
    fn from(data: &'a [u16; SPM_PAGESIZE_WORDS]) -> &'a DataPage {
        unsafe { core::mem::transmute(data) }
    }
}

impl<'a> From<&'a [u8; SPM_PAGESIZE_BYTES]> for &'a DataPage {
    fn from(data: &'a [u8; SPM_PAGESIZE_BYTES]) -> &'a DataPage {
        unsafe { core::mem::transmute(data) }
    }
}

impl From<[u16; SPM_PAGESIZE_WORDS]> for DataPage {
    fn from(data: [u16; SPM_PAGESIZE_WORDS]) -> DataPage {
        DataPage(data)
    }
}

impl From<[u8; SPM_PAGESIZE_BYTES]> for DataPage {
    fn from(data: [u8; SPM_PAGESIZE_BYTES]) -> DataPage {
        unsafe { core::mem::transmute(data) }
    }
}

impl Deref for DataPage {
    type Target = [u16; SPM_PAGESIZE_WORDS];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
