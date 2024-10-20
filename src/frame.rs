//! Provides functionality related to VDIF data frames.

use crate::header::VDIFHeader;
use crate::parsing::{VDIF_HEADER_BYTESIZE, VDIF_HEADER_SIZE};
use crate::payload::VDIFPayload;

/// A VDIF Data Frame. Consists of a [`VDIFHeader`] and a [`VDIFPayload`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VDIFFrame {
    header: VDIFHeader,
    payload: VDIFPayload,
}

impl VDIFFrame {
    /// Construct a new [`VDIFFrame`] from a [`VDIFHeader`] and a [`VDIFPayload`].
    pub fn new(header: VDIFHeader, payload: VDIFPayload) -> Self {
        return Self {
            header: header,
            payload: payload,
        };
    }

    /// Return the total size of this data frame in 32-bit words, including the header **and** payload.
    pub fn wordsize(&self) -> usize {
        return self.payload.wordsize() + VDIF_HEADER_SIZE;
    }

    /// Return the total size of this data frame in bytes, including the header **and** payload.
    pub fn bytesize(&self) -> usize {
        return self.payload.bytesize() + VDIF_HEADER_BYTESIZE;
    }

    /// Return the size of the payload in 32-bit words.
    pub fn payload_wordsize(&self) -> usize {
        return self.payload.wordsize();
    }

    /// Return the size of the payload in bytes.
    pub fn payload_bytesize(&self) -> usize {
        return self.payload.bytesize();
    }

    /// Returns a reference to the [`VDIFHeader`] owned by this data frame.
    pub fn get_header(&self) -> &VDIFHeader {
        return &self.header;
    }

    /// Returns a reference to the [`VDIFPayload`] owned by this data frame.
    pub fn get_payload(&self) -> &VDIFPayload {
        return &self.payload;
    }

    /// Returns a reference to underlying [`u32`] data of the payload.
    pub fn get_payload_data(&self) -> &[u32] {
        return self.payload.get_ref();
    }

    /// Consume `self` and transfer ownership of its components to the user.
    pub fn unpack(self) -> (VDIFHeader, VDIFPayload) {
        return (self.header, self.payload);
    }

    /// Consume `self` and return a VDIF encoded byte slice representing this [`VDIFFrame`].
    pub fn encode(self) -> Box<[u8]> {
        let encoded_header = self.header.encode();
        let encoded_payload = self.payload.encode();

        let mut out: Box<[u8]> = vec![0; encoded_payload.len()+encoded_header.len()].into_boxed_slice();

        let hdr = &mut out[0..32];
        hdr.copy_from_slice(&encoded_header);
        let pld = &mut out[32..];
        pld.copy_from_slice(&encoded_payload);

        return out
    }
}

impl std::fmt::Display for VDIFFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{parsing::parse_frame, VDIFHeader, VDIFPayload};

    use super::VDIFFrame;

    #[test]
    fn test_frame_encode() {
        // Make an example frame
        let example_header = VDIFHeader {
            is_valid: true,
            is_legacy: false,
            time: 100,
            epoch: 10,
            frame: 500,
            version: 3,
            channels: 16,
            size: 40,
            is_real: true,
            bits_per_sample: 8,
            thread: 64,
            station: 50764,
            edv0: 0,
            edv1: 0,
            edv2: 0,
            edv3: 0,
        };
        let example_words: Box<[u32]> = vec![20; 2].into_boxed_slice();
        let example_payload = VDIFPayload::new(example_words);
        let example_frame = VDIFFrame::new(example_header, example_payload);

        // Encode the example into an array of bytes
        let encoded_cpy = example_frame.clone().encode();

        // Then reconstruct a frame by parsing those bytes.
        let (_, parsed_frame) = parse_frame(&encoded_cpy).unwrap();

        assert_eq!(example_frame, parsed_frame)
    }
}
