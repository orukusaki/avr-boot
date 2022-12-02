fn main() {
    println!("cargo:rustc-link-arg=-lc");
    // println!("cargo:rustc-link-arg=-Wl,--section-start=.text=0x7C00,-lc");

    let info = get_mcu_info();

    println!("cargo:rustc-env=AVR_BOOT_SPMCSR={}", info.spmcsr);
    println!("cargo:rustc-env=AVR_BOOT_PAGE_ERASE={}", info.page_erase);
    println!("cargo:rustc-env=AVR_BOOT_PAGE_WRITE={}", info.page_write);
    println!("cargo:rustc-env=AVR_BOOT_PAGE_FILL={}", info.page_fill);

    if let Some(rww_enable) = info.rww_enable {
        println!("cargo:rustc-env=AVR_BOOT_RWW_ENABLE={}", rww_enable);
        println!("cargo:rustc-cfg=rww_enable");
    }

    println!(
        "cargo:rustc-env=AVR_BOOT_LOCK_BITS_SET={}",
        info.boot_lock_bits_set
    );
    println!("cargo:rustc-env=AVR_BOOT_SPM_PAGESIZE={}", info.page_size);
    if let Some(rampz) = info.rampz {
        println!("cargo:rustc-cfg=extended_addressing");
        println!("cargo:rustc-env=AVR_RAMPZ={}", rampz);
    }
}

struct McuInfo {
    spmcsr: u8,
    page_erase: u8,
    page_write: u8,
    page_fill: u8,
    rww_enable: Option<u8>,
    boot_lock_bits_set: u8,
    rampz: Option<u8>,
    page_size: usize,
}

fn get_mcu_info() -> McuInfo {
    const DEFAULT_MCU_FOR_NON_AVR_DOCS: &str = "atmega1280";

    let current_mcu = if avr_mcu::current::is_compiling_for_avr() {
        avr_mcu::current::mcu().expect("no target cpu specified")
    } else {
        avr_mcu::microcontroller(DEFAULT_MCU_FOR_NON_AVR_DOCS)
    };

    let spm_reg = current_mcu
        .registers()
        .find(|r| r.name == "SPMCSR" || r.name == "SPMCR")
        .expect("could not find SPMCSR or SPMCR register");

    let spm_enable = spm_reg
        .bitfields
        .iter()
        .find(|b| b.name == "SPMEN" || b.name == "SELFPRGEN")
        .expect("could not find spm enable bitfield");

    let page_erase = spm_reg
        .bitfields
        .iter()
        .find(|b| b.name == "PGERS")
        .expect("could not find page erase bitfield");

    let page_write = spm_reg
        .bitfields
        .iter()
        .find(|b| b.name == "PGWRT")
        .expect("could not find page write bitfield");

    let rww_enable = spm_reg
        .bitfields
        .iter()
        .find(|b| b.name == "ASRE" || b.name == "RWWSRE");

    let blb_set = spm_reg
        .bitfields
        .iter()
        .find(|b| b.name == "LBSET" || b.name == "BLBSET" || b.name == "RFLB")
        .expect("could not find blb set bitfield");

    let prog_space = current_mcu
        .device
        .address_spaces
        .iter()
        .find(|space| space.name == "prog")
        .expect("could not find prog address space");

    let page_size = prog_space
        .segments
        .iter()
        .find(|seg| seg.name == "FLASH")
        .and_then(|seg| seg.page_size)
        .expect("failed to find page size");

    let rampz = if prog_space.size > 0xffff {
        current_mcu
            .registers()
            .find(|r| r.name == "RAMPZ" || r.name == "RAMPZ0")
            .map(|r| r.offset as u8)
    } else {
        None
    };

    McuInfo {
        spmcsr: spm_reg.offset as u8,
        page_erase: (spm_enable.mask | page_erase.mask) as u8,
        page_write: (spm_enable.mask | page_write.mask) as u8,
        page_fill: spm_enable.mask as u8,
        rww_enable: rww_enable.map(|e| (e.mask | spm_enable.mask) as u8),
        boot_lock_bits_set: (spm_enable.mask | blb_set.mask) as u8,
        rampz,
        page_size: page_size as usize,
    }
}
