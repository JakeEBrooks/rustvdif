use std::io::{Error, Result};

use crate::header::VDIFHeader;

pub struct VDIFPayload {
    words: Box<[u32]>
}

impl VDIFPayload {
    pub fn new(words: Box<[u32]>) -> Self {
        return Self{words: words}
    }

    pub fn frombytes(bytes: Box<[u8]>, header: &VDIFHeader) -> Result<Self> {
        // Check that the correct number of bytes was passed in
        let payload_size = header.payload_bytesize() as usize;
        let bytes_size = bytes.len();
        if bytes_size != payload_size {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("The VDIF header indicates a paylaod of {} bytes, but {} bytes were supplied", payload_size, bytes.len())))
        }

        let num_words = bytes_size / 4;
        // Allocate memory for the payload data
        let mut payload: Box<[u32]> = vec![0; num_words].into_boxed_slice();
        // Make chunks of 4 bytes
        let mut iter = bytes.chunks(4);

        // Iterate over all 4 byte chunks, converting them into u32 and storing in the payload
        for i in 0..num_words {
            payload[i] = u32::from_le_bytes(iter.next().unwrap().try_into().unwrap());
        }

        return Ok(VDIFPayload::new(payload))
    }

    pub fn size(&self) -> usize {
        return self.words.len()
    }

    pub fn bytesize(&self) -> usize {
        return self.size()*4
    }

    pub fn get_ref(&self) -> &[u32] {
        return &self.words
    }
}