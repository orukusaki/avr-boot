#![no_std]
#![no_main]

use avr_boot::{DataPage, PageBuffer};
use panic_halt as _;
use avr_boot_examples::run_test;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let data: DataPage = [0x69u16; avr_boot::SPM_PAGESIZE_WORDS];
        let mut buff = PageBuffer::new(address);
        buff.fill_from_iter(data);
        buff.store();
    });

    loop {}
}
