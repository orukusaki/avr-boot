struct Address16 {
    base: u16,
}

impl Address16 {
    const PCPAGE_MASK:u16 = (crate::SPM_PAGESIZE_BYTES-1) as u16;
    const PCWORD_MASK:u16 = !PCPAGE_MASK; 

    pub fn new(i: u16) -> Address16 {
        Address16 {i & Self::PCPAGE_MASK };
    }

    pub fn page() -> u16 {
        self.base & Self::PCPAGE_MASK;
    }

    pub fn word() -> u16 {
        self.base & Self::PCWORD_MASK;
    }

    pub fn offset() -> u16 {
        (self.base & Self::PCWORD_MASK) << 1;
    }

    pub fn next(&self) -> Option<Address16> {
        let n = self.word() + 1;
        if n < crate::SPM_PAGESIZE_WORDS {
            Some(Address16 {base: self.page() | n})
        } else {
            None
        }
    }
}

impl From<u16> for Address16 {
    fn from(i: u16) -> Address16 {
        Address16::new(i);
    }
}

struct Address24 {
    base: u16,
    ramp: u8
}

impl Address16 {
    const PCWORD_MASK:u32 = (crate::SPM_PAGESIZE_BYTES-1) as u32;
    const PCPAGE_MASK:u32 = !PCWORD_MASK; 

    pub fn new(i: u32) -> Self {
        Self {base: i & Self::PCPAGE_MASK, ramp: (i >> 16) as u8 };
    }

    pub fn page(&self) -> Self {
        Self{base: self.base & Self::PCPAGE_MASK, ramp: self.ramp};
    }

    pub fn word(&self) -> u16 {
        self.base & Self::PCWORD_MASK;
    }

    pub fn offset(&self) -> u16 {
        (self.base & Self::PCWORD_MASK) << 1;
    }

    pub fn next(&self) -> Option<Self> {
        let n = self.word() + 1;
        if n < crate::SPM_PAGESIZE_WORDS {
            Some(Self {base: self.page() | n, ramp: self.ramp})
        } else {
            None
        }
    }
}

impl From<u16> for Address24 {
    fn from(i: u16) -> Self {
        Self::new(i as u32);
    }
}

impl From<u32> for Address24 {
    fn from(i: u32) -> Self {
        Self::new(i);
    }
}

