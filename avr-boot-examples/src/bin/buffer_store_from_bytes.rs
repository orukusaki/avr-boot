#![no_std]
#![no_main]

use avr_boot::PageBuffer;
use avr_boot_examples::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let mut data = [0u8; PageBuffer::LEN * 2];
        data.iter_mut()
            .enumerate()
            .for_each(|(n, b)| *b = if n % 2 == 0 { 0x69 } else { 0x00 });

        let buff = PageBuffer::new(address.into());
        buff.store_from_bytes(&data);
    });

    loop {}
}
