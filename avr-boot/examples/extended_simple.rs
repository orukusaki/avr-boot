#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(asm_sym)]

mod runner;

use avr_boot::{spm, SPM_PAGESIZE_WORDS};
use panic_halt as _;
use runner::extended::run_test;

#[arduino_hal::entry]
fn main() -> ! {
    run_test(|address| {
        for w in 0..SPM_PAGESIZE_WORDS {
            spm::fill_page(address + (w * 2) as spm::Address, 0x69);
        }

        spm::erase_page(address);
        spm::busy_wait();
        spm::write_page(address);
        spm::busy_wait();
        spm::rww_enable();
    });
    loop {}
}
