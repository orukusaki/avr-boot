#![no_std]
#![no_main]

use avr_boot::PageBuffer;
use avr_boot_examples::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let data = [0x69];
        let buff = PageBuffer::new(address.into());
        buff.fill_from_iter(data.into_iter().cycle());
        buff.store();
    });

    loop {}
}
