#![no_std]
#![no_main]

use avr_boot::PageBuffer;
use panic_halt as _;
use avr_boot_examples::run_test;

#[avr_device::entry]
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
