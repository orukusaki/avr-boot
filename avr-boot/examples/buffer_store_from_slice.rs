#![no_std]
#![no_main]

mod runner;

use avr_boot::{DataPage, PageBuffer};
use panic_halt as _;
use runner::run_test;

#[arduino_hal::entry]
fn main() -> ! {
    run_test(|address| {
        let data: DataPage = core::array::from_fn(|_| 0x69);
        let buff = PageBuffer::new(address);
        buff.store_from_slice(&data);
    });

    loop {}
}
