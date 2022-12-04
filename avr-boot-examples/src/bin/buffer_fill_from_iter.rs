#![no_std]
#![no_main]

use avr_boot::PageBuffer;
use panic_halt as _;
use avr_boot_examples::run_test;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let data = [0x69];
        let mut buff = PageBuffer::new(address);
        buff.fill_from_iter(data.into_iter().cycle());
        buff.store();
    });

    loop {}
}