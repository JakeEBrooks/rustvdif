//! Provides functionality related to VDIF data frames.

use crate::header::{VDIFHeader, VDIF_HEADER_BYTESIZE, VDIF_HEADER_SIZE};
use crate::payload::VDIFPayload;

/// A VDIF Data Frame. Consists of a [`VDIFHeader`] and a [`VDIFPayload`].
pub struct VDIFFrame {
    header: VDIFHeader,
    payload: VDIFPayload
}

impl VDIFFrame {
    /// Construct a new [`VDIFFrame`] from a [`VDIFHeader`] and a [`VDIFPayload`].
    pub fn new(header: VDIFHeader, payload: VDIFPayload) -> Self {
        return Self{header: header, payload: payload}
    }

    /// Return the total size of this data frame in 32-bit words, including the header **and** payload.
    pub fn wordsize(&self) -> usize {
        return self.payload.wordsize() + VDIF_HEADER_SIZE
    }

    /// Return the total size of this data frame in bytes, including the header **and** payload.
    pub fn bytesize(&self) -> usize {
        return self.payload.bytesize() + VDIF_HEADER_BYTESIZE
    }

    /// Return the size of the payload in 32-bit words.
    pub fn payload_wordsize(&self) -> usize {
        return self.payload.wordsize()
    }

    /// Return the size of the payload in bytes.
    pub fn payload_bytesize(&self) -> usize {
        return self.payload.bytesize()
    }

    /// Returns a reference to the [`VDIFHeader`] owned by this data frame.
    pub fn get_header(&self) -> &VDIFHeader {
        return &self.header
    }

    /// Returns a reference to the [`VDIFPayload`] owned by this data frame.
    pub fn get_payload(&self) -> &VDIFPayload {
        return &self.payload
    }

    /// Returns a reference to underlying [`u32`] data of the payload.
    pub fn get_payload_data(&self) -> &[u32] {
        return self.payload.get_ref()
    }

    /// Consume `self` and transfer ownership of its components to the user.
    pub fn unpack(self) -> (VDIFHeader, VDIFPayload) {
        return (self.header, self.payload)
    }
}

impl std::fmt::Display for VDIFFrame {
    // FIXME: Merge the time and ref epoch fields into one datetime
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, Payload Size: {}]", self.header, self.payload.bytesize())
    }
}

// TODO: implement VDIFFrameSet, which accumulates VDIFFrames and can calculate quantities like data rate.