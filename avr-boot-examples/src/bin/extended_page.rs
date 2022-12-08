#![no_std]
#![no_main]

use avr_boot::{spm, SPM_PAGESIZE_WORDS};
use avr_boot_examples::extended::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let data = [0x69u16; SPM_PAGESIZE_WORDS];
        spm::store_page(address, &data);
    });
    loop {}
}
