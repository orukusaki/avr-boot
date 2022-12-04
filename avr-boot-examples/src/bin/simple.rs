#![no_std]
#![no_main]

use avr_boot::{spm, SPM_PAGESIZE_WORDS};
use avr_boot_examples::run_test;
use panic_halt as _;

#[avr_device::entry]
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