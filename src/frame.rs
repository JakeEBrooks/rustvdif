use crate::header::{VDIFHeader, VDIF_HEADER_BYTESIZE, VDIF_HEADER_SIZE};
use crate::payload::VDIFPayload;

/// A VDIF Data Frame.
pub struct VDIFFrame {
    pub(crate) header: VDIFHeader,
    pub(crate) payload: VDIFPayload
}

impl VDIFFrame {
    /// Construct a new [`VDIFFrame`] from a [`VDIFHeader`] and a byte slice.
    pub fn new(header: VDIFHeader, payload: VDIFPayload) -> Self {
        return Self{header: header, payload: payload}
    }

    /// Return the total size of this data frame in 32-bit words, including the header and payload.
    pub fn size(&self) -> usize {
        return self.payload.size() + VDIF_HEADER_SIZE
    }

    /// Return the total size of this data frame in bytes, including the header and payload.
    pub fn bytesize(&self) -> usize {
        return self.payload.bytesize() + VDIF_HEADER_BYTESIZE
    }

    pub fn payload_size(&self) -> usize {
        return self.payload.size()
    }

    pub fn payload_bytesize(&self) -> usize {
        return self.payload.bytesize()
    }

    /// Returns a reference to the [`VDIFHeader`] owned by this data frame.
    pub fn get_header(&self) -> &VDIFHeader {
        return &self.header
    }

    /// Returns a reference to a byte array representing the [`VDIFHeader`] of this data frame.
    pub fn get_header_data(&self) -> &[u32; VDIF_HEADER_SIZE] {
        return self.header.get_ref()
    }

    /// Returns a reference to the payload of bytes.
    pub fn get_payload(&self) -> &VDIFPayload {
        return &self.payload
    }

    pub fn get_payload_data(&self) -> &[u32] {
        return self.payload.get_ref()
    }

    /// Consume `self` and transfer ownership of its components out of this [`VDIFFrame`] to the user.
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