#![no_std]
#![no_main]

mod runner;

use avr_boot::PageBuffer;
use panic_halt as _;
use runner::run_test;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let mut buff = PageBuffer::new(address);
        buff.fill_from_fn(|_| 0x69);
        buff.store();
    });

    loop {}
}
