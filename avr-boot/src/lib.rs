//! # Avr Boot
//!
//! This crate contains functions to write to the program memory of Avr MCUs, using the `spm` instruction.
//!
//! It could be considered a reimagining of the macros in boot.h from avr-libc
//! If you're looking to create a bootloader, this crate should be useful.
//!
//! It is hal independent, and optimised for code size. Written in pure Rust plus some artisanal, hand-crafted asm
//!
//! Among the huge range of Avr MCUs, there are many variations on the storage process.
//! It is essential you read the documentation for your target MCU carefully.
//! This crate aims to support as many MCUs as possible, abstracting over their differences wherever practical
//!
//! Supported MCUs:
//! Most Atmega and Attiny
//!
//! Unsupported MCUs:
//!
//! At the moment there is no support for chips that use the newer 'NVM' controller"
//! * attiny10x, attiny10/20/40, attiny4/5/9
//! * attiny8xx, attiny4xx, attiny16xx, attiny32xx, attiny2xx, attiny2xx
//! * atmega80x, atmega48xx, atmega32xx, atmega16xx
//!
//! And some MCUs don't support self-programming at all:
//! * attiny10/12/15, attiny828
//!
//! It supports regular and extended (>64k) addressing modes

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
