#![no_std]
#![no_main]

use avr_boot::{spm, SPM_PAGESIZE_WORDS};
use avr_boot_examples::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|page_address: u16| {
        for w in 0..SPM_PAGESIZE_WORDS {
            let address = page_address + (w * 2) as u16;
            spm::fill_page(address, 0x69);
        }
        spm::erase_page(page_address);
        spm::write_page(page_address);
        spm::rww_enable();
    });

    loop {}
}
