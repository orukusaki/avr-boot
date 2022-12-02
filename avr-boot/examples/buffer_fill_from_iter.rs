#![no_std]
#![no_main]

mod runner;

use avr_boot::PageBuffer;
use panic_halt as _;
use runner::run_test;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let mut data_iterator = [0x69].into_iter().cycle();
        let mut buff = PageBuffer::new(address);
        buff.fill_from_iter(&mut data_iterator);
        buff.store();
    });

    loop {}
}
