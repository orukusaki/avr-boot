#![no_std]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(asm_sym)]

#[cfg(feature = "hal-atmega")]
use atmega_hal as hal;

#[cfg(feature = "hal-attiny")]
use attiny_hal as hal;

use avr_boot::{spm, SPM_PAGESIZE_WORDS};

#[allow(dead_code)]
pub fn run_test<F: FnOnce(spm::Address) -> ()>(f: F) {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let mut signal_pin = pins.pb0.into_output();
    signal_pin.set_low();

    let address: spm::Address = 0x700;

    f(0x700);

    for w in 0..SPM_PAGESIZE_WORDS {
        let read_address = address + (w * 2) as spm::Address;
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

    use avr_boot::{spm_extended, RAMPZ, SPM_PAGESIZE_WORDS};
    use core::arch::asm;

    pub fn run_test<F: FnOnce(spm_extended::Address) -> ()>(f: F) {
        let dp = hal::Peripherals::take().unwrap();
        let pins = hal::pins!(dp);

        let mut signal_pin = pins.pb0.into_output();
        signal_pin.set_low();

        let address: spm_extended::Address = 0x10000;

        f(address);

        for w in 0..SPM_PAGESIZE_WORDS {
            let read_address = address + (w * 2) as spm_extended::Address;
            let word = read_value_extended(read_address);
            if word != 0x69 {
                loop {}
            }
        }

        signal_pin.set_high();
    }

    fn read_value_extended(read_address: spm_extended::Address) -> u16 {
        let out: u16;
        unsafe {
            core::ptr::write_volatile(RAMPZ, (read_address >> 16) as u8);
            asm!(
                "
                elpm {out:l}, Z+
                elpm {out:h}, Z+
                ",
                out = out(reg_pair) out,
                in("Z") read_address as u16,
            );
        }

        out
    }
}
