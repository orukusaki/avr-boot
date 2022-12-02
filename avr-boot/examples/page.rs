#![no_std]
#![no_main]

mod runner;

use avr_boot::{spm, DataPage};
use panic_halt as _;
use runner::run_test;

#[arduino_hal::entry]
fn main() -> ! {
    run_test(|address| {
        let data: DataPage = core::array::from_fn(|_| 0x69);
        spm::store_page(address, &data);
    });

    loop {}
}
