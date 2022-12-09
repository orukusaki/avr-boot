//! # Avr Boot
//!
//! This crate contains functions to write to the program memory of AVR MCUs, using the `spm` instruction.
//!
//! It could be considered a reimagining of the macros in boot.h from avr-libc, but in Rust, plus some artisanal, hand-crafted asm.
//! If you're looking to create a bootloader, this crate should be useful.
//!
//! It is hal independent, and optimised for code size.
//!
//! Regular and extended (>64k) addressing modes are supported.
//!
//! ## Getting started
//! Add the module to your Cargo.toml:
//! ```toml
//! [dependencies]
//! avr-boot = "0.1.0"
//! ```
//!
//! Pick from the high level API:
//! ```rust
//! use avr_boot::PageBuffer;
//!
//! let address: u16 = 0x1000;
//! let data = [0xffff; PageBuffer::LENGTH];
//! let buff = PageBuffer::new(address);
//! buff.copy_from(&data);
//! buff.store();
//! ```
//!
//! Or the low level one:
//! ```rust
//! use avr_boot::{spm, SPM_PAGESIZE_WORDS, Address};
//!
//! let page_address: u16 = 0x1000;
//! for w in 0..SPM_PAGESIZE_WORDS {
//!     spm::fill_page((page_address + (w * 2) as u16), 0x1234);
//! }
//! spm::erase_page(page_address);
//! spm::write_page(page_address);
//! spm::rww_enable();
//! ```
//!
//! Check out the [examples module](https://github.com/orukusaki/avr-boot/tree/main/avr-boot-examples/src/bin) for more usage examples

#![no_std]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(asm_sym)]

mod address;
mod buffer;

use core::ops::Deref;
use const_env__value::value_from_env;
pub mod spm;
pub use address::Address;
pub use buffer::PageBuffer;

/// Total size of the SPM page buffer, for the current MCU target
pub const SPM_PAGESIZE_BYTES: usize = value_from_env!("AVR_BOOT_SPM_PAGESIZE": usize);

/// Total length in 16 but words of the SPM page buffer, for the current MCU target
pub const SPM_PAGESIZE_WORDS: usize = SPM_PAGESIZE_BYTES / 2;

#[cfg(any(extended_addressing))]
#[doc(hidden)]
pub const RAMPZ: *mut u8 = value_from_env!("AVR_RAMPZ": u8) as *mut u8;
#[cfg(target_arch = "avr")]
const SPMCSR: *mut u8 = value_from_env!("AVR_BOOT_SPMCSR": u8) as *mut u8;
#[cfg(target_arch = "avr")]
const PAGE_ERASE: u8 = value_from_env!("AVR_BOOT_PAGE_ERASE": u8);
#[cfg(target_arch = "avr")]
const PAGE_WRITE: u8 = value_from_env!("AVR_BOOT_PAGE_WRITE": u8);
#[cfg(target_arch = "avr")]
const PAGE_FILL: u8 = value_from_env!("AVR_BOOT_PAGE_FILL": u8);
#[cfg(target_arch = "avr")]
const LOCK_BITS_SET: u8 = value_from_env!("AVR_BOOT_LOCK_BITS_SET": u8);
#[cfg(rww_enable)]
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
        unsafe { core::mem::transmute(data) }
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
