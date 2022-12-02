#![no_std]
#![no_main]

mod runner;

use avr_boot::PageBuffer;
use panic_halt as _;
use runner::run_test;

#[arduino_hal::entry]
fn main() -> ! {
    run_test(|address| {
        let mut buff = PageBuffer::new(address);

        for w in &mut buff {
            w.set(0x69);
        }

        buff.store();
    });

    loop {}
}
