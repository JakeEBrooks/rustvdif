//! Provides functionality for interacting with VDIF headers and header information.
//! 
//! Note that functions and methods returning various sizes refer to the encoded VDIF data frame,
//! not the decoded [`VDIFFrame`](super::frame::VDIFFrame) type.

pub(crate) const VDIF_HEADER_SIZE:      usize = 8;
pub(crate) const VDIF_HEADER_BYTESIZE:  usize = 32;

/// A VDIF data frame header containing all the information defined in the VDIF specification.
/// 
/// The header information is accessed through the public fields within the object, and through various methods.
pub struct VDIFHeader {
    /// Whether the frame is valid.
    pub is_valid: bool,
    /// Whether the frame is a legacy VDIF data frame.
    pub is_legacy: bool,
    /// The raw timestamp of the frame
    pub time: u32,
    /// The raw reference epoch of the frame.
    pub epoch: u8,
    /// The frame number.
    pub frame: u32,
    /// The VDIF version.
    pub version: u8,
    /// The number of channels within the frame.
    pub channels: u8,
    /// The size in bytes of the data frame (header **and** payload).
    pub size: u32,
    /// Whether the encoded data is real or complex.
    pub is_real: bool,
    /// The bits/sample of the encoded data.
    pub bits_per_sample: u8,
    /// The thread ID of the frame.
    pub thread: u16,
    /// The source station of the frame.
    pub station: u16,

    /// EDV word 0.
    pub edv0: u32,
    /// EDV word 1.
    pub edv1: u32,
    /// EDV word 2.
    pub edv2: u32,
    /// EDV word 3.
    pub edv3: u32
}

impl VDIFHeader {
    /// Return the size in bytes of the associated data frame.
    pub fn bytesize(&self) -> u32 {
        return self.size
    }

    /// Return the size in bytes of the payload associated with this header
    pub fn payload_bytesize(&self) -> u32 {
        return self.size - (VDIF_HEADER_BYTESIZE as u32)
    }

    /// Return the size in 32-bit words of the associated data frame
    pub fn wordsize(&self) -> u32 {
        return self.size / 4
    }

    /// Return the size in 32-bit words of the payload associated with this header.
    pub fn payload_wordsize(&self) -> u32 {
        return self.wordsize() - (VDIF_HEADER_SIZE as u32)
    }
}

impl std::fmt::Display for VDIFHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Valid: {}, Time: {}s, Frame: {}, Channels: {}, Size: {}, Real: {}, Bits/sample: {}, Thread: {}, Station: {})",
        self.is_valid, self.time, self.frame, self.channels, self.size, self.is_real, self.bits_per_sample, self.thread, self.station)
    }
}