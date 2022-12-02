# Avr Boot

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


It supports regular and extended (>64k) addressing modes

## Getting started

Pick from the high level API:
```rust
use avr_boot::{DataPage, PageBuffer};

let page_address: spm::Address = 0x1000;
let data: DataPage = core::array::from_fn(|_| 0x1234);
let buff = PageBuffer::new(page_address);
buff.store_from_slice(&data);
```

Or the low level one:
```rust
use avr_boot::{spm, SPM_PAGESIZE_WORDS};

let page_address = 0x1000;
for w in 0..SPM_PAGESIZE_WORDS {
    spm::fill_page(0x1000 + (w * 2) as spm::Address, 0x1234);
}
spm::erase_page(page_address);
spm::busy_wait();

spm::write_page(page_address);
spm::busy_wait();

spm::rww_enable();
```

Check out the `avr-boot/examples` folder for more usage examples 

## Supported MCUs:
### Physically tested with:
* atmega328p
* atmega644
* attiny85

### Simulated test using simavr:

atmega1280, atmega1281, atmega1284, atmega1284p, atmega128rfr2, atmega164p, atmega164pa, atmega168, atmega168p, atmega168pa, atmega2560, atmega2561, atmega324a, atmega324p, atmega324pa, atmega328, atmega328p, atmega48, atmega48p, atmega48pa, atmega644, atmega644p, atmega88, atmega88p, atmega88pa

### Library will at least build for:
at90can128, at90can32, at90can64, at90pwm1, at90pwm161, at90pwm216, at90pwm2b, at90pwm316, at90pwm3b, at90pwm81, at90usb1286, at90usb1287, at90usb162, at90usb646, at90usb647, at90usb82, ata5272, ata5505, ata5782, ata5790, ata5790n, ata5791, ata5795, ata5831, ata6285, ata6286, ata6612c, ata6613c, ata6614q, ata6617c, ata664251, ata8210, ata8510, atmega128, atmega1280, atmega1281, atmega1284, atmega1284p, atmega1284rfr2, atmega128a, atmega128rfa1, atmega128rfr2, atmega16, atmega162, atmega164a, atmega164p, atmega164pa, atmega165a, atmega165p, atmega165pa, atmega168, atmega168a, atmega168p, atmega168pa, atmega168pb, atmega169a, atmega169p, atmega169pa, atmega16a, atmega16hva, atmega16hvb, atmega16hvbrevb, atmega16m1, atmega16u2, atmega16u4, atmega2560, atmega2561, atmega2564rfr2, atmega256rfr2, atmega32, atmega324a, atmega324p, atmega324pa, atmega324pb, atmega325, atmega3250, atmega3250a, atmega3250p, atmega3250pa, atmega325a, atmega325p, atmega325pa, atmega328, atmega328p, atmega328pb, atmega329, atmega3290, atmega3290a, atmega3290p, atmega3290pa, atmega329a, atmega329p, atmega329pa, atmega32a, atmega32c1, atmega32hvb, atmega32hvbrevb, atmega32m1, atmega32u2, atmega32u4, atmega406, atmega48, atmega48a, atmega48p, atmega48pa, atmega48pb, atmega64, atmega640, atmega644, atmega644a, atmega644p, atmega644pa, atmega644rfr2, atmega645, atmega6450, atmega6450a, atmega6450p, atmega645a, atmega645p, atmega649, atmega6490, atmega6490a, atmega6490p, atmega649a, atmega649p, atmega64a, atmega64c1, atmega64hve2, atmega64m1, atmega64rfr2, atmega8, atmega8515, atmega8535, atmega88, atmega88a, atmega88p, atmega88pa, atmega88pb, atmega8a, atmega8hva, atmega8u2, attiny13, attiny13a, attiny1634, attiny167, attiny2313, attiny2313a, attiny24, attiny24a, attiny25, attiny261, attiny261a, attiny4313, attiny43u, attiny44, attiny441, attiny44a, attiny45, attiny461, attiny461a, attiny48, attiny84, attiny841, attiny84a, attiny85, attiny861, attiny861a, attiny87, attiny88

However they are all missing support from either `avr-hal` or `simavr`, so I've not been able to test any further

### Unsupported MCUs:

At the moment there is no support for chips that use the newer 'NVM' controller:
* attiny10x, attiny10/20/40, attiny4/5/9
* attiny8xx, attiny4xx, attiny16xx, attiny32xx, attiny2xx, attiny2xx
* atmega80x, atmega48xx, atmega32xx, atmega16xx

And some MCUs don't support self-programming at all:
* attiny10/12/15, attiny828
