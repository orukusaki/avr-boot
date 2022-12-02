# Avr Boot

This crate contains functions to write to the program memory of Avr MCUs, using the `spm` instruction.

It could be considered a reimagining of the macros in boot.h from avr-libc
If you're looking to create a bootloader, this crate should be useful.

It is hal independent, and optimised for code size. Written in pure Rust plus some artisanal, hand-crafted asm

Among the huge range of Avr MCUs, there are many variations on the storage process.
It is essential you read the documentation for your target MCU carefully.
This crate aims to support as many MCUs as possible, abstracting over their differences wherever practical

Supported MCUs:
Most Atmega and Attiny

Unsupported MCUs:

At the moment there is no support for chips that use the newer 'NVM' controller:
* attiny10x, attiny10/20/40, attiny4/5/9
* attiny8xx, attiny4xx, attiny16xx, attiny32xx, attiny2xx, attiny2xx
* atmega80x, atmega48xx, atmega32xx, atmega16xx

And some MCUs don't support self-programming at all:
* attiny10/12/15, attiny828

It supports regular and extended (>64k) addressing modes
