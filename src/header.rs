//! Functionality for interacting with VDIF headers and header information.
//! 
//! This module gives you easy access to VDIF header information by providing a variety
//! of convenience methods for accessing the fields defined in the VDIF specification.

pub(crate) const VDIF_HEADER_SIZE:      usize = 8;
pub(crate) const VDIF_HEADER_BYTESIZE:  usize = 32;

const MASK_IS_VALID:        u32 = 0b10000000000000000000000000000000;
const MASK_IS_LEGACY:       u32 = 0b01000000000000000000000000000000;
const MASK_TIME:            u32 = 0b00111111111111111111111111111111;
const MASK_REF_EPOCH:       u32 = 0b00111111000000000000000000000000;
const MASK_FRAME_NO:        u32 = 0b00000000111111111111111111111111;
const MASK_VERSION_NO:      u32 = 0b11100000000000000000000000000000;
const MASK_LOG2_CHANNELS:   u32 = 0b00011111000000000000000000000000;
const MASK_BYTE_SIZE:       u32 = 0b00000000111111111111111111111111;
const MASK_IS_REAL:         u32 = 0b10000000000000000000000000000000;
const MASK_BITS_PER_SAMPLE: u32 = 0b01111100000000000000000000000000;
const MASK_THREAD_ID:       u32 = 0b00000011111111110000000000000000;
const MASK_STATION_ID:      u32 = 0b00000000000000001111111111111111;

/// Represents a station identifier as either a [`String`] or a [`u32`] ID number.
/// 
/// In the VDIF standard stations can be identified as the commonly 
/// used two character station code (e.g. "Lo" for Lovell or "Ef" for Effelsberg)
/// or as a simple numeric ID.
pub enum StationID {
    /// The two character station code.
    Name(String),
    /// The numeric station ID.
    ID(u32)
}

/// A fixed size object representing a VDIF header.
/// 
/// This object is the primary way of interacting with VDIF headers, and has many methods for extracting
/// information from the header.
#[derive(Debug, Default)]
pub struct VDIFHeader {
    words: [u32; VDIF_HEADER_SIZE]
}

impl VDIFHeader {
    /// Construct a new [`VDIFHeader`] from a fixed size array of [`u32`]s.
    pub fn new(words: [u32; VDIF_HEADER_SIZE]) -> Self {
        return Self {words: words}
    }

    /// Construct a new [`VDIFHeader`] from a fixed size array of bytes, arranged according to the VDIF standard.
    pub fn frombytes(bytes: [u8; VDIF_HEADER_BYTESIZE]) -> Self {
        // I can't see any case where these panic
        let w0: u32 = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let w1: u32 = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let w2: u32 = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let w3: u32 = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let w4: u32 = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        let w5: u32 = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
        let w6: u32 = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        let w7: u32 = u32::from_le_bytes(bytes[28..32].try_into().unwrap());

        return Self{words: [w0, w1, w2, w3, w4, w5, w6, w7]}
    }

    /// If the associated data frame is valid, return `true`, else return `false`.
    pub fn is_valid(&self) -> bool {
        return self.extract_with_mask(0, MASK_IS_VALID) == 0
    }

    /// Logically opposite to [`VDIFHeader::is_valid`].
    pub fn is_invalid(&self) -> bool {
        return self.extract_with_mask(0, MASK_IS_VALID) != 0
    }

    /// If the associated data frame is a legacy VDIF data frame, return `true`, else return `false`. Note that legacy VDIF data
    /// frames are not currently supported.
    pub fn is_legacy(&self) -> bool {
        return self.extract_with_mask(0, MASK_IS_LEGACY) != 0
    }

    /// Get the raw timestamp in seconds of the associated data frame. See the VDIF specification for details.
    pub fn raw_time(&self) -> u32 {
        return self.extract_with_mask(0, MASK_TIME)
    }

    /// Get an unprocessed number representing the reference epoch of the associated data frame.
    pub fn raw_ref_epoch(&self) -> u32 {
        return self.extract_with_mask(1, MASK_REF_EPOCH) >> 24
    }

    /// Get the data frame number of the associated data frame.
    pub fn frame_no(&self) -> u32 {
        return self.extract_with_mask(1, MASK_FRAME_NO)
    }

    /// Get the VDIF version the associated data frame adheres to.
    pub fn version_no(&self) -> u32 {
        return self.extract_with_mask(2, MASK_VERSION_NO) >> 29
    }

    /// Get the channel field without performing exponentiation.
    pub fn raw_channel_no(&self) -> u32 {
        return self.extract_with_mask(2, MASK_LOG2_CHANNELS) >> 24
    }

    /// Get the channel number of the associated data frame.
    pub fn channel_no(&self) -> u32 {
        return 2u32.pow(self.extract_with_mask(2, MASK_LOG2_CHANNELS) >> 24)
    }

    /// Get the total size in bytes of the associated data frame, including *both* the header *and* the payload.
    pub fn bytesize(&self) -> u32 {
        return self.extract_with_mask(2, MASK_BYTE_SIZE) * 8
    }

    /// Get the total size in bytes of the associated payload.
    pub fn payload_bytesize(&self) -> u32 {
        return self.bytesize() - VDIF_HEADER_BYTESIZE as u32
    }

    /// Return `true` if the associated data frame carries real data, and `false` if it is complex.
    pub fn is_real(&self) -> bool {
        return self.extract_with_mask(3, MASK_IS_REAL) == 0
    }

    /// Logically opposite to [`VDIFHeader::is_real`].
    pub fn is_complex(&self) -> bool {
        return self.extract_with_mask(3, MASK_IS_REAL) != 0
    }

    /// Get the bits per sample of the data carried in the associated data frame.
    pub fn bits_per_sample(&self) -> u32 {
        return self.extract_with_mask(3, MASK_BITS_PER_SAMPLE) >> 26
    }
    
    /// Get the thread ID of the associated data frame.
    pub fn thread_id(&self) -> u32 {
        return self.extract_with_mask(3, MASK_THREAD_ID) >> 16
    }

    /// Get the station identifier of the associated data frame as a numeric identifier.
    pub fn station_id(&self) -> u32 {
        return self.extract_with_mask(3, MASK_STATION_ID)
    }

    /// Get a reference to the underlying byte data of this header.
    pub fn get_ref(&self) -> &[u32; VDIF_HEADER_SIZE] {
        return &self.words
    }

    /// Extract a copy of the desired 32-bit word.
    /// This a low-level method you probably shouldn't use.
    pub fn extract(&self, word: usize) -> u32 {
        return self.words[word]
    }

    /// Extract a copy of the desired 32-bit word and mask it accordingly.
    /// This a low-level method you probably shouldn't use.
    pub fn extract_with_mask(&self, word: usize, mask: u32) -> u32 {
        return self.extract(word) & mask
    }

}

impl std::fmt::Display for VDIFHeader {
    // FIXME: Merge the time and ref epoch fields into one datetime
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Valid: {}, Time: {}s, Frame: {}, Channels: {}, Size: {}, Real: {}, Bits/sample: {}, Thread: {}, Station ID: {})",
                self.is_valid(), self.raw_time(),
                self.frame_no(), self.channel_no(), self.bytesize(), self.is_real(),
                self.bits_per_sample(), self.thread_id(), self.station_id())
    }
}