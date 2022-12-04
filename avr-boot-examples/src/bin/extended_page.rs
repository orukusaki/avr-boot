#![no_std]
#![no_main]

use avr_boot::{spm, DataPage, SPM_PAGESIZE_WORDS};
use panic_halt as _;
use avr_boot_examples::extended::run_test;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let data: DataPage = [0x69; SPM_PAGESIZE_WORDS];
        spm::store_page(address, &data);
    });
    loop {}
}
