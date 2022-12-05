#![no_std]
#![no_main]

use avr_boot::{spm, DataPage, SPM_PAGESIZE_WORDS};
use avr_boot_examples::extended::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let data: DataPage = [0x69; SPM_PAGESIZE_WORDS];
        spm::store_page(address, &data);
    });
    loop {}
}
