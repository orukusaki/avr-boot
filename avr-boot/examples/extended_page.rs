#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(asm_sym)]

mod runner;

use avr_boot::{spm, DataPage, SPM_PAGESIZE_WORDS};
use panic_halt as _;
use runner::extended::run_test;

#[arduino_hal::entry]
fn main() -> ! {
    run_test(|address| {
        let data: DataPage = [0x69; SPM_PAGESIZE_WORDS];
        spm::store_page(address, &data);
    });
    loop {}
}
