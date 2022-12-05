#![no_std]
#![no_main]

use avr_boot::{DataPage, PageBuffer};
use avr_boot_examples::run_test;
use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    run_test(|address| {
        let data: DataPage = core::array::from_fn(|_| 0x69);
        let buff = PageBuffer::new(address);
        buff.store_from_slice(&data);
    });

    loop {}
}
