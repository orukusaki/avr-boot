#![no_std]
#![no_main]

use avr_boot::PageBuffer;
use avr_boot_examples::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let mut buff = PageBuffer::new(address);
        buff.fill_from_fn(|_| 0x69);
        buff.store();
    });

    loop {}
}
