//! Implements [`VDIFFrame`].

use crate::header::VDIFHeader;
use crate::header_encoding::decode_header;

/// A VDIF frame.
/// 
/// Each [`VDIFFrame`] simply contains a heap allocated slice of `u32`s. The header is decoded when you call
/// [`get_header`](VDIFFrame::get_header), so you don't pay a cost for simply creating this type.
#[derive(Debug)]
pub struct VDIFFrame {
    data: Box<[u32]>
}

impl VDIFFrame {
    /// Construct a [`VDIFFrame`] from a raw `u32` slice.
    pub fn new(data: Box<[u32]>) -> Self {
        return Self { data: data }
    }

    /// Construct a [`VDIFFrame`] by copying the contents of `data`.
    pub fn from_slice(data: &[u32]) -> Self {
        return Self{ data: Box::from(data) }
    }

    /// Construct a completely empty [`VDIFFrame`].
    pub fn empty(frame_size: usize) -> Self {
        assert!(frame_size % 4 == 0, "VDIF frames must contain an integer number of 32 bit words");
        return Self { data: vec![0; frame_size/4].into_boxed_slice() }
    }

    /// Get a single `u32` word from this frame.
    pub fn get_word(&self, ind: usize) -> u32 {
        return self.data[ind]
    }

    /// Get a single `u32` word from the payload. Equivalent to `get_word(8 + ind)`.
    pub fn get_data_word(&self, ind: usize) -> u32 {
        return self.data[8+ind]
    }

    /// Construct a [`VDIFHeader`] from this frame.
    pub fn get_header(&self) -> VDIFHeader {
        return decode_header(&self)
    }

    /// Get a reference to the payload portion of this frame.
    pub fn get_payload(&self) -> &[u32] {
        return &self.data[8..]
    }

    /// Get a mutable reference to the payload portion of this frame.
    pub fn get_payload_mut(&mut self) -> &mut [u32] {
        return &mut self.data[8..]
    }

    /// Get the length in `u32` words of this frame.
    pub fn len(&self) -> usize {
        return self.data.len()
    }

    /// Get the size in bytes of this frame.
    pub fn bytesize(&self) -> usize {
        return self.len()*4
    }

    /// Return a reference to the underlying `u32` slice, including the header.
    pub fn as_slice(&self) -> &[u32] {
        return &self.data
    }

    /// Return a mutable reference to the underlying `u32` slice, including the header.
    pub fn as_slice_mut(&mut self) -> &mut [u32] {
        return &mut self.data
    }

    /// Return a reference to the underlying bytes, including the header.
    pub fn as_bytes(&self) -> &[u8] {
        #[cfg(target_endian = "big")]
        panic!("RustVDIF does not yet support big endian targets.");
        return unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len()*4)
        }
    }

    /// Return a mutable reference to the underlying bytes, including the header.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        #[cfg(target_endian = "big")]
        panic!("RustVDIF does not yet support big endian targets.");
        return unsafe {
            std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut u8, self.data.len()*4)
        }
    }
}