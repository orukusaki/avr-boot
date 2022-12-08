use core::convert::From;

/// 16 or 24 bit program memory address
///
/// Used internally to provide correct page allignment and efficient storage.
/// Use u16.into() or u32.into() to suit your target MCU's address space size.
/// 
/// Although this struct is always 3 bytes in size, on an MCU with <65kB of flash memory,
/// the highest byte is optimised away completely, taking it's effective size down to only 2 bytes.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Address {
    base: u16,
    ramp: u8,
}

impl Address {
    const PCWORD_MASK: u16 = (crate::SPM_PAGESIZE_BYTES - 1) as u16;
    const PCPAGE_MASK: u16 = !Self::PCWORD_MASK;

    fn new(base: u32) -> Self {
        Self {
            base: base as u16,
            ramp: (base >> 16) as u8,
        }
    }

    /// Mask off the PCWORD part of the address, leaving only PCPAGE.  
    /// 
    /// The resulting address is aligned to the start of the page. 
    pub fn into_page_aligned(self) -> Self {
        Self {
            base: self.base & Self::PCPAGE_MASK,
            ramp: self.ramp,
        }
    }
  
    /// The word byte index within the page: technically PCWORD << 1
    pub fn word(&self) -> u16 {
        self.base & Self::PCWORD_MASK
    }

    /// Create a new address by taking the first address of the page and adding the given offset
    pub fn with_offset(&self, offset: u8) -> Self {
        let aligned = self.into_page_aligned();
        Self {
            base: aligned.base | (offset as u16),
            ramp: self.ramp,
        }
    }

    /// The extended byte of the address, usually written to RAMPZ on MCUs with extended addressing
    pub fn ramp(&self) -> u8 {
        self.ramp
    }
}

impl From<u16> for Address {
    fn from(i: u16) -> Self {
        Self::new(i as u32)
    }
}

impl From<u8> for Address {
    fn from(i: u8) -> Self {
        Self::new(i as u32)
    }
}

impl From<u32> for Address {
    fn from(i: u32) -> Self {
        Self::new(i)
    }
}

impl From<Address> for u32 {
    fn from(address: Address) -> u32 {
        address.base as u32 + ((address.ramp as u32) << 16)
    }
}

impl From<Address> for u16 {
    fn from(address: Address) -> u16 {
        address.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_aligns_addess_to_page() {
        let start_address = crate::SPM_PAGESIZE_BYTES as u32 * 8 + 17;
        let address: Address = start_address.into();

        assert_eq!(
            address.into_page_aligned(),
            Address::new(crate::SPM_PAGESIZE_BYTES as u32 * 8)
        );
    }

    #[test]
    fn it_masks_pcword_part() {
        let start_address = crate::SPM_PAGESIZE_BYTES as u32 * 8 + 17;
        let address: Address = start_address.into();

        assert_eq!(address.word(), 17);
    }

    #[test]
    fn it_adds_offset_to_page_base() {
        let start_address = crate::SPM_PAGESIZE_BYTES as u32 * 8 + 17;
        let address: Address = start_address.into();

        assert_eq!(
            address.with_offset(5),
            (crate::SPM_PAGESIZE_BYTES as u32 * 8 + 5).into()
        );
    }
}
