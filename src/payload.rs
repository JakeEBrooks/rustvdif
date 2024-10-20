//! Provides functionality related to VDIF payloads.

/// A VDIF payload, consisting of a series of [`u32`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Consume `self` and return a VDIF encoded array of bytes representing this payload.
    pub fn encode(self) -> Box<[u8]> {
        let mut out: Box<[u8]> = vec![0; self.bytesize()].into_boxed_slice();

        for i in 0..self.wordsize() {
            let bytes: [u8; 4] = self.words[i].to_le_bytes();
            out[i*4] = bytes[0];
            out[i*4 + 1] = bytes[1];
            out[i*4 + 2] = bytes[2];
            out[i*4 + 3] = bytes[3];
        }

        return out
    }
}
