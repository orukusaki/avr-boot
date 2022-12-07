//! High level page buffer API

use crate::Address;
use crate::{spm, DataPage};

/// Representation of the spm page buffer.
///
/// The page buffer is a special area of memory which is write-only, and only writable using the `spm` instruction.
/// Setting a value in the buffer does not write to the program memory - that only happens when [`PageBuffer::store`] is called
///
/// # Example
/// ```no_run
/// let address:u16 = 0x1000;
/// let buff = PageBuffer::new(address.into());
/// for w in buff.iter() {
///     w.set(0xabcd);
/// }
/// buff.store();
/// ```
///
/// A whole page is written in one go, so if you only want to change part of a page, you need to make sure you have
/// loaded the rest of the page into the buffer first.
///
/// There is only one physical buffer in the system, so you should make sure only one of these structs ever
/// exists at any time. This rule is not enforced.
///
/// The page address will be aligned downwards to the nearest starting page address
///
/// Note: The `store_from_*` methods are generally slightly quicker and smaller than the `fill_from_*` equivalents,
/// but require you to already have the whole page stored in RAM somewhere
///
pub struct PageBuffer {
    address: Address,
}

/// A word cell in the page buffer
pub struct BufferCell {
    offset: u8,
}

impl PageBuffer {
    pub const LEN: usize = crate::SPM_PAGESIZE_WORDS;

    /// Create a new PageBuffer with the given address.
    ///
    /// # Example
    /// ```rust
    /// let buff = PageBuffer::new(0x101f);
    /// assert_eq!(0x1000, buff.address().into());
    /// ```
    /// The page address will be aligned downwards to the nearest starting page address
    pub fn new(address: Address) -> PageBuffer {
        PageBuffer {
            address: address.into_page_aligned(),
        }
    }

    /// Get the base page address to be operated on
    ///
    /// # Example
    /// ```rust
    /// let buff = PageBuffer::new(0x1000);
    /// assert_eq!(Address::new(0x1000), buff.address());
    /// ```
    /// The page address will be aligned downwards to the nearest starting page address
    pub fn address(&self) -> Address {
        self.address
    }

    /// Fill the buffer from a slice, and write it immediately
    ///
    /// # Example
    ///
    /// ```no_run
    /// let data = [0xffff; PageBuffer::LEN];
    /// let buff = PageBuffer::new(address);
    /// buff.store_from_slice(&data);
    /// ```
    pub fn store_from_slice(self, data: &DataPage) {
        spm::store_page(self.address, data);
    }

    /// Fill the buffer from a byte slice, and write it immediately
    ///
    /// # Example
    ///
    /// ```no_run
    /// let data = [0xff; avr_boot::SPM_PAGESIZE_BYTES];
    /// let buff = PageBuffer::new(address);
    /// buff.store_from_bytes(&data);
    /// ```
    pub fn store_from_bytes(self, data: &[u8; crate::SPM_PAGESIZE_BYTES]) {
        let data_words: &[u16; Self::LEN] = unsafe { core::mem::transmute(data) };
        self.store_from_slice(data_words);
    }

    /// Fill the buffer by repeatedly calling the callback function
    ///
    /// # Example
    ///
    /// ```no_run
    /// let buff = PageBuffer::new(address);
    /// buff.fill_from_fn(|offset| offset);
    /// buff.store();
    /// ```
    pub fn fill_from_fn<F>(&self, f: F)
    where
        F: Fn(u8) -> u16,
    {
        for word in self.iter() {
            word.set(f(word.offset));
        }
    }

    /// Fill the buffer by repeatedly polling an iterator.  
    ///
    /// # Example
    ///
    /// ```no_run
    /// let data = [0x69];
    /// let i = data.iter().cycle();
    /// let page_address = 0x1000;
    /// let buff = PageBuffer::new(page_address.into());
    /// buff.fill_from_iter(i);
    /// buff.store();
    /// /// ```
    pub fn fill_from_iter(&self, i: impl IntoIterator<Item = u16>) {
        for (word, value) in self.iter().zip(i.into_iter()) {
            word.set(value);
        }
    }

    /// Erase the page from program memory, then write the contents of the buffer to it
    pub fn store(self) {
        spm::erase_page(self.address);
        spm::busy_wait();

        spm::write_page(self.address);
        spm::busy_wait();
        spm::rww_enable();
    }

    /// Iterate the buffer as writable [`BufferCell`]s
    ///
    /// # Example
    ///
    /// ```no_run
    /// let buff = PageBuffer::new(address);
    /// for w in buff.iter() {
    ///     w.set(0x69);
    /// }
    /// buff.store();
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = BufferCell> {
        CellIter { offset: 0 }
    }
}

struct CellIter {
    offset: u8,
}

impl Iterator for CellIter {
    type Item = BufferCell;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.offset;
        if current >= crate::SPM_PAGESIZE_BYTES as u8 {
            None
        } else {
            self.offset += 2;
            Some(BufferCell { offset: current })
        }
    }
}

/// A single 16 bit word in the page buffer. Write only.
impl BufferCell {
    /// Set the value of the cell
    pub fn set(&self, w: u16) {
        spm::fill_page(Address::new(self.offset.into()), w);
    }
}
