//! Low level API for calling bootloader functions

use crate::Address;
use crate::*;

use cfg_if::cfg_if;
#[allow(unused_imports)]
use core::arch::asm;

/// Store a whole page into program memory by erasing the page, filling the buffer,
/// and writing the buffer to the program memory.  
/// `address` must be page aligned.
pub fn store_page<'a>(address: impl Into<Address>, data: impl Into<&'a DataPage>) {
    let page_address: Address = address.into();

    erase_page(page_address);
    copy_to_buffer(data);
    write_page(page_address);
    rww_enable();
}

/// Erase the page from program memory
///
/// The PCPAGE part of the address is used to address the page, the PCWORD part must be zero
#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
pub fn erase_page(address: impl Into<Address>) {
    let page_address: Address = address.into();
    let z_address: u16 = page_address.into_page_aligned().into();

    busy_wait();
    rampz(page_address.ramp());
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "
                    out {SPMCSR} r24
                    spm
                    ",
                    in("r24") PAGE_ERASE,
                    in("Z") z_address,
                    SPMCSR = const SPMCSR_ADDR - 0x20,
                );
            }
        }
    }
}

/// Write data to the page buffer
///
/// Only the PCWORD part of the address actually matters, the size of which varies according to SPM_PAGESIZE_BYTES
#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
pub fn fill_page(address: impl Into<Address>, data: u16) {
    let page_address: Address = address.into();
    let z_address: u16 = page_address.into();

    busy_wait();
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "
                    movw r0 {data}
                    out {SPMCSR} r24
                    spm
                    eor	r1, r1
                    ",
                    data = in(reg_iw) data,
                    in("r24") PAGE_FILL,
                    in("Z") z_address,
                    SPMCSR = const SPMCSR_ADDR - 0x20,
                )
            }
        }
    }
}

/// Write the page from the buffer to the program memory
///
/// The PCPAGE part of the address is used to address the page, the PCWORD part must be zero
#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
pub fn write_page(address: impl Into<Address>) {
    let page_address: Address = address.into();
    let z_address: u16 = page_address.into_page_aligned().into();

    busy_wait();
    rampz(page_address.ramp());
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "
                    out {SPMCSR} r24
                    spm
                    ",
                    in("r24") PAGE_WRITE,
                    in("Z") z_address,
                    SPMCSR = const SPMCSR_ADDR - 0x20,
                )
            }
        }
    }
}

/// Fill the whole buffer at once
///
#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
pub fn copy_to_buffer<'a>(data: impl Into<&'a DataPage>) {
    busy_wait();
    rampz(0);
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "
                    1:                       
                        ld      r0,         X+  // Load r0r1 pair with data from X pointer
                        ld      r1,         X+
                        out {SPMCSR} r24
                        spm                     // call spm(PAGE_FILL) (r24 is always 1st byte argument)
                        adiw    Z,          2   // increment Z
                        subi    {words},    1   // decrement counter
                        brne    1b              // loop until counter reaches 0

                        clr	    r1
                    ",

                    words = inout(reg) SPM_PAGESIZE_WORDS as u8 => _,
                    in("r24") PAGE_FILL,
                    inout("X") data.into().as_ptr() => _,
                    inout("Z") 0u16 => _,
                    SPMCSR = const SPMCSR_ADDR - 0x20,
                )
            }
        }
    }
}

#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
pub fn lock_bits_set(lock_bits: u8) {
    rampz(0);
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            let value = !lock_bits;
            unsafe {
                asm!(
                    "
                    mov r0 {value}
                    out {SPMCSR} r24
                    spm
                    ",
                    value = in(reg) value,
                    in("r24") LOCK_BITS_SET,
                    in("Z") 0x0001u16,
                    SPMCSR = const SPMCSR_ADDR - 0x20,
                )
            }
        }
    }
}

/// Re-enable the RWW section after programming, to enable it to be read
#[cfg(rww_enable)]
pub fn rww_enable() {
    busy_wait();
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "
                    out {SPMCSR} r24
                    spm
                    ",
                    in("r24") RWW_ENABLE,
                    SPMCSR = const SPMCSR_ADDR - 0x20,
                );
            }
        }
    }
}

/// Empty function for devices without a RWW section
#[cfg(not(rww_enable))]
pub fn rww_enable() {}

/// Wait for the current SPM operation to complete.
///
/// On devices with a RWW section, the CPU is not halted during the SPM operation if the RWW section is being written to.
/// Therefore it is important that we make sure the operation is complete before trying to do the next operation.
pub fn busy_wait() {
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
           while unsafe { core::ptr::read_volatile(SPMCSR) } & PAGE_FILL != 0 {}
        }
    }
}

#[cfg_attr(
    not(all(target_arch = "avr", extended_addressing)),
    allow(unused_variables)
)]
fn rampz(value: u8) {
    cfg_if! {
        if #[cfg(all(target_arch = "avr", extended_addressing, not(doc)))] {
            unsafe {
                core::ptr::write_volatile(crate::RAMPZ, value);
            }
        }
    }
}
