//! Provides functionality related to VDIF payloads.

/// A VDIF payload, consisting of a series of [`u32`]s.
pub struct VDIFPayload {
    words: Box<[u32]>,
}

impl VDIFPayload {
    /// Construct a new [`VDIFPayload`] from a [`Box<u32>`] object.
    pub fn new(words: Box<[u32]>) -> Self {
        return Self { words: words };
    }

    /// Return the size in 32-bit words of this payload.
    pub fn wordsize(&self) -> usize {
        return self.words.len();
    }

    /// Return the size in bytes of this payload.
    pub fn bytesize(&self) -> usize {
        return self.wordsize() * 4;
    }

    /// Return a reference to underlying [`u32`] slice.
    pub fn get_ref(&self) -> &[u32] {
        return &self.words;
    }
}
