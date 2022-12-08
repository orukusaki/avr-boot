//! High level page buffer API

use crate::Address;
use crate::{spm, DataPage};
use core::iter;

/// Representation of the spm page buffer.
///
/// The page buffer is a special area of memory which is write-only, and only writable using the `spm` instruction.
/// Setting a value in the buffer does not write to the program memory - that only happens when [`PageBuffer::store`] is called
///
/// # Example
/// ```no_run
/// use avr_boot::PageBuffer;
///
/// let address:u16 = 0x1000;
/// let buff = PageBuffer::new(address);
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

impl PageBuffer {
    pub const LEN: usize = crate::SPM_PAGESIZE_WORDS;

    /// Create a new PageBuffer with the given address.
    ///
    /// # Example
    /// ```rust
    /// use avr_boot::PageBuffer;
    ///
    /// let buff = PageBuffer::new(0x101fu16);
    /// assert_eq!(0x1000u16, buff.address().into());
    /// ```
    /// The page address will be aligned downwards to the nearest starting page address
    pub fn new(address: impl Into<Address>) -> PageBuffer {
        PageBuffer {
            address: address.into().into_page_aligned(),
        }
    }

    /// Get the base page address to be operated on
    ///
    /// # Example
    /// ```rust
    /// use avr_boot::{PageBuffer, Address};
    ///
    /// let buff = PageBuffer::new(0x1000u16);
    /// assert_eq!(Address::new(0x1000), buff.address());
    /// ```
    /// The page address will be aligned downwards to the nearest starting page address
    pub fn address(&self) -> Address {
        self.address
    }

    /// Fill the buffer from a slice
    ///
    /// # Example
    ///
    /// ```no_run
    /// use avr_boot::{DataPage, PageBuffer};
    ///
    /// let address: u16 = 0x1000;
    /// let data = DataPage(core::array::from_fn(|_| 0x69));
    /// let buff = PageBuffer::new(address);
    /// buff.copy_from(&data);
    /// buff.store();
    /// ```
    ///
    /// # Example
    ///
    /// ```no_run
    /// use avr_boot::PageBuffer;
    ///
    /// let address: u16 = 0x1000;
    /// let data = [0xff; avr_boot::SPM_PAGESIZE_BYTES];
    /// let buff = PageBuffer::new(address);
    /// buff.copy_from(&data);
    /// buff.store();
    /// ```
    pub fn copy_from<'a>(&self, data: impl Into<&'a DataPage>) {
        spm::copy_to_buffer(data);
    }

    /// Fill the buffer from a slice and store it immediately
    ///
    /// # Example
    ///
    /// ```no_run
    /// use avr_boot::{DataPage, PageBuffer};
    ///
    /// let address: u16 = 0x1000;
    /// let data = DataPage(core::array::from_fn(|_| 0x69));
    /// let buff = PageBuffer::new(address);
    /// buff.copy_from(&data);
    /// buff.store();
    /// ```
    ///
    pub fn store_from<'a>(self, data: impl Into<&'a DataPage>) {
        spm::erase_page(self.address);
        spm::copy_to_buffer(data);

        spm::busy_wait();
        spm::write_page(self.address);
    }

    /// Fill the buffer by repeatedly calling the callback function
    ///
    /// # Example
    ///
    /// ```no_run
    /// use avr_boot::PageBuffer;
    ///
    /// let address:u16 = 0x1000;
    /// let buff = PageBuffer::new(address);
    /// buff.fill_from_fn(|| Some(0x1234));
    /// buff.store();
    /// ```
    pub fn fill_from_fn<F>(&self, f: F)
    where
        F: FnMut() -> Option<u16>,
    {
        self.fill_from_iter(iter::from_fn(f));
    }

    /// Fill the buffer by repeatedly polling an iterator.  
    ///
    /// # Example
    ///
    /// ```no_run
    /// use avr_boot::PageBuffer;
    /// use core::iter;
    ///
    /// let page_address:u16 = 0x1000;
    /// let buff = PageBuffer::new(page_address);
    /// buff.fill_from_iter(iter::repeat(0x69));
    /// buff.store();
    /// ```
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
    }

    /// Iterate the buffer as writable [`BufferCell`]s
    ///
    /// # Example
    ///
    /// ```no_run
    /// use avr_boot::PageBuffer;
    ///
    /// let address: u16 = 0x1000;
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

impl Drop for PageBuffer {
    // Wait for any current spm operation to complete and
    // re-enable the rww section (if there is one)
    fn drop(&mut self) {
        spm::busy_wait();
        spm::rww_enable();
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
pub struct BufferCell {
    offset: u8,
}

impl BufferCell {
    /// Set the value of the word in the spm buffer
    pub fn set(&self, w: u16) {
        spm::fill_page(Address::new(self.offset.into()), w);
    }
}
