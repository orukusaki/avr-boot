use core::convert::From;

/// 16 or 24 bit program memory address
///
/// Used internally to provide correct page allignment and efficient storage.
/// Use u16.into() or u32.into() to suit your target MCU's address space size
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

    pub fn into_page_aligned(self) -> Self {
        Self {
            base: self.base & Self::PCPAGE_MASK,
            ramp: self.ramp,
        }
    }

    /// The word byte index: technically PCWORD << 1
    pub fn word(&self) -> u16 {
        self.base & Self::PCWORD_MASK
    }

    pub fn with_offset(&self, offset: u8) -> Self {
        let aligned = self.into_page_aligned();
        Self {
            base: aligned.base | (offset as u16),
            ramp: self.ramp,
        }
    }

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
