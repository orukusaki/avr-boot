//! # Avr Boot
//!
//! This crate contains functions to write to the program memory of AVR MCUs, using the `spm` instruction.
//!
//! It could be considered a reimagining of the macros in boot.h from avr-libc, but in Rust, plus some artisanal, hand-crafted asm
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
//! use avr_boot::{DataPage, PageBuffer};
//!
//! let page_address: spm::Address = 0x1000;
//! let data: DataPage = core::array::from_fn(|_| 0x1234);
//! let buff = PageBuffer::new(page_address);
//! buff.store_from_slice(&data);
//! ```
//!
//! Or the low level one:
//! ```rust
//! use avr_boot::{spm, SPM_PAGESIZE_WORDS};
//!
//! let page_address = 0x1000;
//! for w in 0..SPM_PAGESIZE_WORDS {
//!     spm::fill_page(0x1000 + (w * 2) as spm::Address, 0x1234);
//! }
//! spm::erase_page(page_address);
//! spm::busy_wait();
//!
//! spm::write_page(page_address);
//! spm::busy_wait();
//!
//! spm::rww_enable();
//! ```
//!
//! Check out the [examples module](https://github.com/orukusaki/avr-boot/tree/main/avr-boot-examples/src/bin) for more usage examples

#![no_std]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(asm_sym)]

use const_env__value::value_from_env;

pub const SPM_PAGESIZE_BYTES: usize = value_from_env!("AVR_BOOT_SPM_PAGESIZE": usize);
pub const SPM_PAGESIZE_WORDS: usize = SPM_PAGESIZE_BYTES / 2;

const SPMCSR: *mut u8 = value_from_env!("AVR_BOOT_SPMCSR": u8) as *mut u8;
const PAGE_ERASE: u8 = value_from_env!("AVR_BOOT_PAGE_ERASE": u8);
const PAGE_WRITE: u8 = value_from_env!("AVR_BOOT_PAGE_WRITE": u8);
const PAGE_FILL: u8 = value_from_env!("AVR_BOOT_PAGE_FILL": u8);
const LOCK_BITS_SET: u8 = value_from_env!("AVR_BOOT_LOCK_BITS_SET": u8);

#[cfg(rww_enable)]
const RWW_ENABLE: u8 = value_from_env!("AVR_BOOT_RWW_ENABLE": u8);

/// An array of memory the same size as the page buffer
pub type DataPage = [u16; SPM_PAGESIZE_WORDS];

#[cfg(any(extended_addressing))]
pub const RAMPZ: *mut u8 = value_from_env!("AVR_RAMPZ": u8) as *mut u8;

pub mod spm_extended;
pub mod spm_normal;

/// Will link to either spm_normal or spm_extended depending on the target
#[cfg(extended_addressing)]
pub use spm_extended as spm;

/// Will link to either spm_normal or spm_extended depending on the target
#[cfg(not(extended_addressing))]
pub use spm_normal as spm;

pub mod address;

#[cfg(not(extended_addressing))]
pub use address::Address16 as Address;

#[cfg(extended_addressing)]
pub use address::Address24 as Address;

pub mod buffer;

pub type PageBuffer = buffer::PageBuffer;
