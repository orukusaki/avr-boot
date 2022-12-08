#![no_std]
#![no_main]

use avr_boot::{spm, SPM_PAGESIZE_WORDS};
use avr_boot_examples::extended::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|page_address: u32| {
        for w in 0..SPM_PAGESIZE_WORDS {
            let address = page_address + (w * 2) as u32;
            spm::fill_page(address, 0x69);
        }

        spm::erase_page(page_address);
        spm::busy_wait();
        spm::write_page(page_address);
        spm::busy_wait();
        spm::rww_enable();
    });
    loop {}
}
