pub use crate::*;
use core::arch::asm;

pub type Address = u16;

pub fn store_page(address: Address, data: &DataPage) {
    erase_page(address);
    busy_wait();
    fill_page_buffer(address, data);
    write_page(address);
    busy_wait();
    rww_enable();
}

pub fn erase_page(address: Address) {
    unsafe {
        asm!(
            "rcall {spm}",
            spm = sym spm,
            in("r24") PAGE_ERASE,
            in("Z") address,
        );
    }
}

pub fn fill_page(address: Address, data: u16) {
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
            in("Z") address,
        )
    }
}

pub fn write_page(address: Address) {
    unsafe {
        asm!(
            "rcall {spm}",
            spm = sym spm,
            in("r24") PAGE_WRITE,
            in("Z") address,
        )
    }
}

pub fn clear_buffer(_address: Address) {
    unimplemented!();
    // unsafe {
    //     asm!(
    //         "rcall {spm}",
    //         spm = sym spm,
    //         in("r24") PAGE_WRITE,
    //         in("Z") address,
    //     )
    // }
}

/// Fill the whole buffer at once
///
/// If have the data in a RAM buffer already, this is slightly smaller
/// and faster than using [`fill_page`] in a loop
pub fn fill_page_buffer(address: Address, data: &DataPage) {
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
            inout("X") data.as_ptr() => _,
            inout("Z") address => _,
        )
    }
}

pub fn lock_bits_set(lock_bits: u8) {
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

#[cfg(rww_enable)]
pub fn rww_enable() {
    spm(RWW_ENABLE);
}

#[cfg(not(rww_enable))]
pub fn rww_enable() {}

pub fn busy_wait() {
    while unsafe { core::ptr::read_volatile(SPMCSR) } & PAGE_FILL != 0 {}
}

pub(crate) extern "C" fn spm(spmcsr_val: u8) {
    unsafe {
        core::ptr::write_volatile(SPMCSR, spmcsr_val);
        asm!("spm")
    }
}