use crate::{decoding::header::{decode_bits_per_sample_1, decode_frameno, decode_is_real, decode_is_valid, decode_log2channels, decode_ref_epoch, decode_size8, decode_stationid, decode_threadid, decode_time}, header_masks::{MASK_IS_VALID, MASK_SIZE8}, VDIFHeader};

// It would be kind of insane to create a 8MB frame, so if the user tried to do this
// something has probably gone wrong 
const MAX_FRAME_SIZE: u32 = 8000000;

/// A VDIF frame.
///
/// Each [`VDIFFrame`] simply contains a heap allocated slice of `u32`s.
#[derive(Debug)]
pub struct VDIFFrame {
    data: Box<[u32]>,
}

impl VDIFFrame {
    /// Construct a [`VDIFFrame`] from a raw `u32` slice.
    pub fn new(data: Box<[u32]>) -> Self {
        assert!(
            data.len() % 2 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self { data: data };
    }

    /// Construct a [`VDIFFrame`] by copying the contents of `data`.
    pub fn from_slice(data: &[u32]) -> Self {
        assert!(
            data.len() % 2 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self {
            data: Box::from(data),
        };
    }

    /// Construct a [`VDIFFrame`] by using the information contained in a [`VDIFHeader`].
    pub fn from_header(header: VDIFHeader) -> Self {
        let size = (header.as_slice()[2] & MASK_SIZE8) * 8;
        assert!(size < MAX_FRAME_SIZE, "Tried to create a VDIF frame larger than 8MB!");
        let mut frame = Self::new_empty(size as usize);
        frame.as_mut_slice()[0..8].copy_from_slice(header.as_slice());
        return frame
    }

    /// Construct a [`VDIFFrame`] by copying the contents of a `&[u8]` byte slice.
    pub fn from_byte_slice(data: &[u8]) -> Self {
        assert!(
            data.len() % 8 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self { data: Box::from(
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) }
        ) }
    }

    /// Construct a completely empty [`VDIFFrame`] with all header and data bytes set to zero.
    pub fn new_empty(frame_size: usize) -> Self {
        assert!(
            frame_size % 8 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self {
            data: vec![0; frame_size / 4].into_boxed_slice(),
        };
    }

    /// Construct a completely empty [`VDIFFrame`] with the invalid bit set, and all other bits set to zero.
    pub fn new_invalid(frame_size: usize) -> Self {
        let mut out = Self::new_empty(frame_size);
        out.as_mut_slice()[0] |= MASK_IS_VALID;
        return out;
    }

    /// Get a reference to the payload portion of this frame.
    pub fn get_payload(&self) -> &[u32] {
        return &self.data[8..];
    }

    /// Get a mutable reference to the payload portion of this frame.
    pub fn get_mut_payload(&mut self) -> &mut [u32] {
        return &mut self.data[8..];
    }

    /// Get the length in `u32` words of this frame.
    pub fn len(&self) -> usize {
        return self.data.len();
    }

    /// Get the size in bytes of this frame.
    pub fn bytesize(&self) -> usize {
        return self.len() * 4;
    }

    /// Return a reference to the underlying `u32` slice, including the header.
    pub fn as_slice(&self) -> &[u32] {
        return &self.data;
    }

    /// Return a mutable reference to the underlying `u32` slice, including the header.
    pub fn as_mut_slice(&mut self) -> &mut [u32] {
        return &mut self.data;
    }

    /// Return a reference to the underlying bytes, including the header.
    pub fn as_bytes(&self) -> &[u8] {
        return unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4)
        };
    }

    /// Return a mutable reference to the underlying bytes, including the header.
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        return unsafe {
            std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut u8, self.data.len() * 4)
        };
    }
}

impl std::fmt::Display for VDIFFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Valid: {}, Time: {}, Epoch: {}, Frame: {}, Chans: {}, Size: {}, Real: {}, Bits per sample: {}, Thread: {}, Station: {}>",
        decode_is_valid(&self),
        decode_time(&self),
        decode_ref_epoch(&self),
        decode_frameno(&self),
        1u8 << decode_log2channels(&self),
        decode_size8(&self)*8,
        decode_is_real(&self),
        decode_bits_per_sample_1(&self)+1,
        decode_threadid(&self),
        decode_stationid(&self))
    }
}