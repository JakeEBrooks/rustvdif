//! Functionality for interacting with VDIF header information.
//! 
//! This module gives you easy access to VDIF header information by providing a variety
//! of convenience methods for accessing the values defined in the VDIF specification.
//! 
//! # Examples
//! The [`VDIFHeader`] object has many methods for accessing VDIF meta data:
//! ```rust
//! let file = File::open("my/vdif/file").unwrap();
//! let mut reader = VDIFReader::new(file).unwrap();
//! let first_header = reader.get_header().unwrap();
//! // Print the size in bytes of the associated data frame.
//! println!("{}", first_header.byte_size());
//! ```

use std::string::FromUtf8Error;

/// The size of a VDIF header.
pub const VDIF_HEADER_SIZE: usize = 32;
const VDIF_WORD_SIZE:       usize = 4;

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
    data: [u8; VDIF_HEADER_SIZE]
}

impl VDIFHeader {
    /// Construct a new [`VDIFHeader`] from a fixed size byte array.
    pub fn new(data: [u8; VDIF_HEADER_SIZE]) -> Self {
        return Self {data: data}
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

    /// Get the timestamp of the associated data frame. See the VDIF specification for details.
    pub fn time(&self) -> u32 {
        return self.extract_with_mask(0, MASK_TIME)
    }

    /// Get an unprocessed number representing the reference epoch of the associated data frame.
    pub fn raw_ref_epoch(&self) -> u32 {
        return self.extract_with_mask(1, MASK_REF_EPOCH) >> 24
    }

    // TODO: return a time::Date representing the reference epoch.

    /// Get the data frame number of the associated data frame.
    pub fn frame_no(&self) -> u32 {
        return self.extract_with_mask(1, MASK_FRAME_NO)
    }

    /// Get the VDIF version the associated data frame adheres to.
    pub fn version_no(&self) -> u32 {
        return self.extract_with_mask(2, MASK_VERSION_NO) >> 29
    }

    //  TODO: Read the spec for what channel number actually means.

    /// Get the channel number of the associated data frame.
    pub fn channel_no(&self) -> u32 {
        return 2u32.pow(self.extract_with_mask(2, MASK_LOG2_CHANNELS) >> 24)
    }

    /// Get the total size of the associated data frame, including *both* the header *and* the payload.
    pub fn byte_size(&self) -> u32 {
        return self.extract_with_mask(2, MASK_BYTE_SIZE) * 8
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

    /// Get the station identifier of the associated data frame as a [`String`].
    pub fn station_str(&self) -> Result<String, FromUtf8Error> {
        // FIXME: make this into something less hard coded. Probably breaks on big endian systems
        let mut buf: [u8; 2] = [0; 2];
        buf.copy_from_slice(&self.data[12..14]);
        buf.reverse();
        return String::from_utf8(buf.to_vec())
    }

    /// Get an enum containing the station two-character identifier if possible, otherwise it contains the numeric station ID.
    pub fn station(&self) -> StationID {
        match self.station_str() {
            Ok(station_string) => StationID::Name(station_string),
            Err(_) => StationID::ID(self.station_id())
        }
    }

    /// Get a reference to the underlying byte data of this header.
    pub fn get_ref(&self) -> &[u8; VDIF_HEADER_SIZE] {
        return &self.data
    }

    /// Extract a copy of the desired 32-bit word.
    /// This a low-level method you probably shouldn't use.
    pub fn extract(&self, word: usize) -> u32 {
        let mut buf: [u8; VDIF_WORD_SIZE] = [0; VDIF_WORD_SIZE];
        let ind = VDIF_WORD_SIZE*word;
        buf.copy_from_slice(&self.data[ind..ind+VDIF_WORD_SIZE]);
        return u32::from_le_bytes(buf)
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
        let station_str = match self.station_str() {
            Ok(idstring) => idstring,
            Err(_) => "N/A".to_string()
        };
        write!(f, "(Valid: {}, Time: {}s, Ref Epoch: {}, Frame: {}, Channel: {}, Size: {}, Real: {}, Bits/sample: {}, Thread: {}, Station: {}, Station ID: {})",
                self.is_valid(), self.time(), self.raw_ref_epoch(),
                self.frame_no(), self.channel_no(), self.byte_size(), self.is_real(),
                self.bits_per_sample(), self.thread_id(), station_str, self.station_id())
    }
}

// /// Construct a [`VDIFHeader`] from the given information.
// pub fn construct(isvalid: &bool,
//                 date: &time::Date,
//                 time: &time::Time,
//                 frameno: &u32,
//                 channelno: &u32,
//                 payloadsize: &u32,
//                 isreal: &bool,
//                 bits_per_sample: &u32,
//                 threadid: &u32,
//                 stationid: &StationID) -> VDIFHeader {
//     return VDIFHeader{data: [0; VDIF_HEADER_SIZE]}
// }