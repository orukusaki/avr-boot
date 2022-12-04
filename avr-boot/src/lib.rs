//! # Avr Boot
//!
//! This crate contains functions to write to the program memory of Avr MCUs, using the `spm` instruction.
//!
//! It could be considered a reimagining of the macros in boot.h from avr-libc
//! If you're looking to create a bootloader, this crate should be useful.
//!
//! It is hal independent, and optimised for code size. Written in pure Rust plus some artisanal, hand-crafted asm
//!
//! Regular and extended (>64k) addressing modes are supported.  See README.md for more, including supported MCU list

#![no_std]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(asm_sym)]
#![feature(associated_type_bounds)]

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

pub type DataPage = [u16; SPM_PAGESIZE_WORDS];

#[cfg(extended_addressing)]
pub const RAMPZ: *mut u8 = value_from_env!("AVR_RAMPZ": u8) as *mut u8;

pub mod spm_extended;
pub mod spm_normal;

/// Will link to either spm_normal or spm_extended depending on the target
#[cfg(extended_addressing)]
pub use spm_extended as spm;

/// Will link to either spm_normal or spm_extended depending on the target
#[cfg(not(extended_addressing))]
pub use spm_normal as spm;

pub mod buffer;
pub type PageBuffer = buffer::PageBuffer<{ SPM_PAGESIZE_WORDS as spm::Address }>;
