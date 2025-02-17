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
            data.len() % 8 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self { data: data };
    }

    /// Construct a [`VDIFFrame`] by copying the contents of `data`.
    pub fn from_slice(data: &[u32]) -> Self {
        assert!(
            data.len() % 8 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self {
            data: Box::from(data),
        };
    }

    /// Construct a completely empty [`VDIFFrame`].
    pub fn empty(frame_size: usize) -> Self {
        assert!(
            frame_size % 8 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self {
            data: vec![0; frame_size / 4].into_boxed_slice(),
        };
    }

    /// Construct a completely empty [`VDIFFrame`] with the invalid bit set.
    pub fn invalid(frame_size: usize) -> Self {
        let mut out = Self::empty(frame_size);
        out.as_mut_slice()[0] |= 0x80000000;
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
