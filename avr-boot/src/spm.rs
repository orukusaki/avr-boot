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

    // wait for erase_page to finish - on mcus with a RWW section,
    // we can fill the buffer while the page is erasing
    busy_wait();
    write_page(page_address);

    // wait for write_page to finish - on mcus with a RWW section,
    // we can fill the buffer while the page is erasing
    busy_wait();
    rww_enable();
}

/// Erase the page from program memory
///
/// The PCPAGE part of the address is used to address the page, the PCWORD part must be zero
#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
pub fn erase_page(address: impl Into<Address>) {
    let page_address: Address = address.into();
    let z_address: u16 = page_address.into_page_aligned().into();

    rampz(page_address.ramp());
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "rcall {spm}",
                    spm = sym spm,
                    in("r24") PAGE_ERASE,
                    in("Z") z_address,
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

    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "
                    movw r0 {data}
                    rcall {spm}
                    eor	r1, r1
                    ",
                    data = in(reg_iw) data,
                    spm = sym spm,
                    in("r24") PAGE_FILL,
                    in("Z") z_address,
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

    rampz(page_address.ramp());
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "rcall {spm}",
                    spm = sym spm,
                    in("r24") PAGE_WRITE,
                    in("Z") z_address,
                )
            }
        }
    }
}

/// Fill the whole buffer at once
///
/// If have the data in a RAM buffer already, this is slightly smaller
/// and faster than using [`fill_page`] in a loop
#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
pub fn copy_to_buffer<'a>(data: impl Into<&'a DataPage>) {
    rampz(0);
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                asm!(
                    "
                    1:                       
                        ld      r0,         X+  // Load r0r1 pair with data from X pointer
                        ld      r1,         X+
                        rcall   {spm}           // call spm(PAGE_FILL) (r24 is always 1st byte argument)
                        adiw    Z,          2   // increment Z
                        subi    {words},    1   // decrement counter
                        brne    1b              // loop until counter reaches 0

                        clr	    r1
                    ",

                    words = inout(reg) SPM_PAGESIZE_WORDS as u8 => _,
                    spm = sym spm,
                    in("r24") PAGE_FILL,
                    inout("X") data.into().as_ptr() => _,
                    inout("Z") 0u16 => _,
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
                    rcall {spm}
                    ",
                    spm = sym spm,
                    value = in(reg) value,
                    in("r24") LOCK_BITS_SET,
                    in("Z") 0x0001u16,
                )
            }
        }
    }
}

/// Re-enable the RWW section after programming, to enable it to be read
#[cfg(rww_enable)]
pub fn rww_enable() {
    spm(RWW_ENABLE);
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

#[cfg_attr(not(target_arch = "avr"), allow(unused_variables))]
extern "C" fn spm(spmcsr_val: u8) {
    cfg_if! {
        if #[cfg(all(target_arch = "avr", not(doc)))] {
            unsafe {
                core::ptr::write_volatile(SPMCSR, spmcsr_val);
                asm!("spm")
            }
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
