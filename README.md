# Avr Boot
[![Rust](https://github.com/orukusaki/avr-boot/actions/workflows/rust.yml/badge.svg)](https://github.com/orukusaki/avr-boot/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/orukusaki/avr-boot/master/LICENSE)
[![docs.rs](https://img.shields.io/docsrs/avr-boot)](https://docs.rs/avr-boot/latest/avr_boot/)

This crate contains functions to write to the program memory of Avr MCUs, using the `spm` instruction.

It could be considered a reimagining of the macros in `boot.h` from avr-libc.
If you're looking to create a bootloader, this crate should be useful.

It is hal independent, and optimised for code size. Written in Rust plus some artisanal, hand-crafted asm.

This crate aims to support as many MCUs as possible, abstracting over their differences wherever practical.  
Among the huge range of Avr MCUs, there are many variations on the self-programming process.
It is essential you read the documentation for your target MCU carefully.  
On many devices the spm instruction will only work if the programming code is in the bootloader section of the flash memory.
On most MCUs the fuses will also need to be set correctly to allow self-programming. 
Interrupts should always be disabled while self-programming.

This crate supports regular and extended (>64k) addressing modes

## Getting started

Add the module to your Cargo.toml:
```toml
[dependencies]
avr-boot = "0.2.3"
```

Pick from the high level API:
```rust
use avr_boot::PageBuffer;

let address: u16 = 0x1000;
let data = [0xffff; PageBuffer::LEN];
let buff = PageBuffer::new(address);
buff.copy_from(&data);
buff.store();
```

Or the low level one:
```rust
use avr_boot::{spm, SPM_PAGESIZE_WORDS, Address};

let address: u16 = 0x1000;
for w in 0..SPM_PAGESIZE_WORDS {
    spm::fill_page((address + (w * 2) as u16), 0x1234);
}
spm::erase_page(address);
spm::write_page(address);
spm::rww_enable();
```

Check out the [examples module](https://github.com/orukusaki/avr-boot/tree/main/avr-boot-examples/src/bin)  for more usage examples 

## Supported MCUs:
### Physically tested with:
* atmega328p
* atmega644
* attiny85

### Simulated test using simavr:

atmega1280, atmega1281, atmega1284, atmega1284p, atmega128rfr2, atmega164p, atmega164pa, atmega168, atmega168p, atmega168pa, atmega2560, atmega2561, atmega324a, atmega324p, atmega324pa, atmega328, atmega328p, atmega48, atmega48p, atmega48pa, atmega644, atmega644p, atmega88, atmega88p, atmega88pa

### Library will at least build for:
at90can128, at90can32, at90can64, at90pwm1, at90pwm161, at90pwm216, at90pwm2b, at90pwm316, at90pwm3b, at90pwm81, at90usb1286, at90usb1287, at90usb162, at90usb646, at90usb647, at90usb82, ata5272, ata5505, ata5782, ata5790, ata5790n, ata5791, ata5795, ata5831, ata6285, ata6286, ata6612c, ata6613c, ata6614q, ata6617c, ata664251, ata8210, ata8510, atmega128, atmega128a, atmega128rfa1, atmega16, atmega162, atmega164a, atmega165a, atmega165p, atmega165pa, atmega168a, atmega168pb, atmega169a, atmega169p, atmega169pa, atmega16a, atmega16hva, atmega16hvb, atmega16hvbrevb, atmega16m1, atmega16u2, atmega16u4, atmega2564rfr2, atmega256rfr2, atmega32, atmega324pb, atmega325, atmega3250, atmega3250a, atmega3250p, atmega3250pa, atmega325a, atmega325p, atmega325pa, atmega328pb, atmega329, atmega3290, atmega3290a, atmega3290p, atmega3290pa, atmega329a, atmega329p, atmega329pa, atmega32a, atmega32c1, atmega32hvb, atmega32hvbrevb, atmega32m1, atmega32u2, atmega32u4, atmega406, atmega48a, atmega48pb, atmega64, atmega640, atmega644a, atmega644pa, atmega644rfr2, atmega645, atmega6450, atmega6450a, atmega6450p, atmega645a, atmega645p, atmega649, atmega6490, atmega6490a, atmega6490p, atmega649a, atmega649p, atmega64a, atmega64c1, atmega64hve2, atmega64m1, atmega64rfr2, atmega8, atmega8515, atmega8535, atmega88a, atmega88pb, atmega8a, atmega8hva, atmega8u2, attiny13, attiny13a, attiny1634, attiny167, attiny2313, attiny2313a, attiny24, attiny24a, attiny25, attiny261, attiny261a, attiny4313, attiny43u, attiny44, attiny441, attiny44a, attiny45, attiny461, attiny461a, attiny48, attiny84, attiny841, attiny84a, attiny85, attiny861, attiny861a, attiny87, attiny88

However they are all missing support from either `avr-hal` or `simavr`, so I've not been able to test further.

### Unsupported MCUs:

At the moment there is no support for chips that use the newer 'NVM' controller:
* attiny10x, attiny10/20/40, attiny4/5/9
* attiny8xx, attiny4xx, attiny16xx, attiny32xx, attiny2xx, attiny2xx
* atmega80x, atmega48xx, atmega32xx, atmega16xx

And some MCUs don't support self-programming at all:
* attiny10/12/15, attiny828

