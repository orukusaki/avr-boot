//! High level page buffer API

use crate::spm::Address;
use crate::{spm, DataPage};

/// Representation of the spm page buffer.
///
/// The page buffer is a special area of memory which is write-only, and only writable using the `spm` instruction.
/// Setting a value in the buffer does not write to the program memory - that only happens when [`PageBuffer::store`] is called
///
/// # Example
/// ```no_run
/// // clear a page
/// let mut buff = PageBuffer::new(address);
/// for w in &mut buff {
///     w.set(0xabcd);
/// }
/// buff.store();
/// ```
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
    const OFFSET_MASK: Address = ((Self::LEN << 1) - 1) as Address;
    const PAGE_MASK: Address = (!Self::OFFSET_MASK);

    /// Create a new PageBuffer with the given address.
    ///
    /// The page address will be aligned downwards to the nearest starting page address
    pub fn new(address: Address) -> PageBuffer {
        PageBuffer {
            address: address & Self::PAGE_MASK,
        }
    }

    /// Get the page address.
    ///
    /// The page address will be aligned downwards to the nearest starting page address
    pub fn address(&self) -> Address {
        self.address & Self::PAGE_MASK
    }

    /// Fill the buffer from a slice, and write it immediately
    ///
    /// # Example
    ///
    /// ```no_run
    /// let data = [0xffff; PageBuffer::LEN];
    /// let mut buff = PageBuffer::new(address);
    /// buff.store_from_slice(&data);
    /// ```
    pub fn store_from_slice(self, data: &DataPage) {
        spm::store_page(self.address(), data);
        // No need to run destructor
        core::mem::forget(self);
    }

    /// Fill the buffer from a byte slice, and write it immediately
    ///
    /// # Example
    ///
    /// ```no_run
    /// let data = [0xff; avr_boot::SPM_PAGESIZE_BYTES];
    /// let mut buff = PageBuffer::new(address);
    /// buff.store_from_bytes(&data);
    /// ```
    pub fn store_from_bytes(self, data: &[u16; crate::SPM_PAGESIZE_BYTES]) {
        let data_words: &[u16; Self::LEN] = unsafe { core::mem::transmute(data) };
        self.store_from_slice(data_words);
    }

    /// Fill the buffer by repeatedly calling the callback function
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut buff = PageBuffer::new(address);
    /// buff.fill_from_fn(|offset| offset);
    /// buff.store();
    /// ```
    pub fn fill_from_fn<F>(&mut self, f: F)
    where
        F: Fn(u8) -> u16,
    {
        for word in self {
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
    ///
    /// let mut buff = PageBuffer::new(address);
    /// buff.fill_from_iter(&mut i);
    /// buff.store();
    /// ```
    pub fn fill_from_iter(&mut self, i: impl IntoIterator<Item = u16>) {
        for (word, value) in self.into_iter().zip(i.into_iter()) {
            word.set(value);
        }
    }

    /// Erase the page from program memory, then write the contents of the buffer to it
    pub fn store(self) {
        let page_address = self.address();
        spm::erase_page(page_address);
        spm::busy_wait();

        spm::write_page(page_address);
        spm::busy_wait();

        // No need to run destructor
        core::mem::forget(self);
    }
}

impl Drop for PageBuffer {
    fn drop(&mut self) {
        // TODO: on some MCUs there is a buffer clear SPM command, run it here
        // clear_buffer(self.address);
        spm::rww_enable();
    }
}

/// Iterate the buffer as writable [`BufferCell`]s
///
/// # Example
///
/// ```no_run
/// let mut buff = PageBuffer::new(address);
/// for w in &mut buff {
///     w.set(0x69);
/// }
/// buff.store();
/// ```
impl Iterator for PageBuffer {
    type Item = BufferCell;

    fn next(&mut self) -> Option<Self::Item> {
        let address = self.address;
        let offset = address & Self::OFFSET_MASK;

        // sneaky bit-stuffing: becuase the buffer entries are always words
        // we have a spare bit in address, and can use this to avoid needing to add
        // an extra value to keep track of the iteration
        if (offset as usize) < Self::LEN {
            self.address = address + 1;
            Some(BufferCell {
                offset: (offset as u8) << 1,
            })
        } else {
            None
        }
    }
}

impl ExactSizeIterator for PageBuffer {
    fn len(&self) -> usize {
        Self::LEN - (self.address & Self::PAGE_MASK) as usize
    }
}

/// A single 16 bit word in the page buffer. Write only.
impl BufferCell {
    /// Set the value of the cell
    pub fn set(&self, w: u16) {
        spm::fill_page(self.offset.into(), w);
    }
}
