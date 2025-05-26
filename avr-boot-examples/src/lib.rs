#![no_std]
#![feature(asm_experimental_arch)]

#[cfg(feature = "hal-atmega")]
use atmega_hal as hal;

#[cfg(feature = "hal-attiny")]
use attiny_hal as hal;

use avr_boot::SPM_PAGESIZE_WORDS;

#[allow(dead_code)]
pub fn run_test<F: FnOnce(u16) -> ()>(f: F) {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let mut signal_pin = pins.pb0.into_output();
    signal_pin.set_low();

    let page_address: u16 = 0x700;

    f(page_address);

    for w in 0..SPM_PAGESIZE_WORDS {
        let read_address: u16 = page_address + (w * 2) as u16;
        let word = unsafe { avr_progmem::raw::read_value(read_address as *const u16) };
        if word != 0x69 {
            loop {}
        }
    }

    signal_pin.set_high();
}

#[cfg(feature = "extended_addressing")]
pub mod extended {

    #[cfg(feature = "hal-atmega")]
    use atmega_hal as hal;

    #[cfg(feature = "hal-attiny")]
    use attiny_hal as hal;

    use avr_boot::{RAMPZ, SPM_PAGESIZE_WORDS};
    use core::arch::asm;

    pub fn run_test<F: FnOnce(u32) -> ()>(f: F) {
        let dp = hal::Peripherals::take().unwrap();
        let pins = hal::pins!(dp);

        let mut signal_pin = pins.pb0.into_output();
        signal_pin.set_low();

        let page_address: u32 = 0x10000;

        f(page_address);

        for w in 0..SPM_PAGESIZE_WORDS {
            let read_address: u32 = page_address + (w * 2) as u32;
            let word = read_value_extended(read_address);
            if word != 0x69 {
                loop {}
            }
        }

        signal_pin.set_high();
    }

    fn read_value_extended(address: u32) -> u16 {
        let out: u16;
        let z_address: u16 = address as u16;
        unsafe {
            core::ptr::write_volatile(RAMPZ, (address >> 16) as u8);
            asm!(
                "
                elpm {out:l}, Z+
                elpm {out:h}, Z+
                ",
                out = out(reg_pair) out,
                in("Z") z_address,
            );
        }

        out
    }
}
