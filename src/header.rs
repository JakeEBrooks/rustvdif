//! Provides functionality for interacting with VDIF headers and header information.

/// Station identifiers can be either a two character ASCII string, or a numeric ID.
pub enum StationID {
    /// The station ID as a string
    StringID(String),
    /// The station ID as a number
    NumericID(u16),
}

/// A VDIF data frame header.
///
/// The header information is accessed through public fields and methods.
#[derive(Debug, Default)]
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
    pub frameno: u32,
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
    pub edv3: u32,
}

impl VDIFHeader {
    /// Get the total size in bytes of the associated VDIF frame.
    pub fn bytesize(&self) -> u32 {
        return self.size
    }

    /// Get the total size in 32-bit words of the associated VDIF frame.
    pub fn wordsize(&self) -> u32 {
        return self.size/4
    }

    /// Get the total size in bytes of the associated VDIF payload.
    pub fn data_bytesize(&self) -> u32 {
        return self.size - 32
    }

    /// Get the total size in 32-bit words of the associated VDIF payload.
    pub fn data_wordsize(&self) -> u32 {
        return (self.size - 32)/4
    }

    /// Return the station ID as either a string or a number.
    ///
    /// This function attempts to find two valid ASCII characters in the station ID field. If it fails it returns a number, otherwise
    /// it returns a two character string. If you know you just want the numeric ID, use `self.station` instead.
    pub fn station(&self) -> StationID {
        // FIXME: double check this actually works on all systems, (I fudged this)
        let bytes = self.station.to_be_bytes();
        let id = String::from_utf8(bytes.to_vec());
        match id {
            Ok(idstring) => StationID::StringID(idstring),
            Err(_) => StationID::NumericID(self.station),
        }
    }
}

impl std::fmt::Display for VDIFHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut station: String = "  ".to_string();
        if let StationID::StringID(str) = self.station() {
            station = str
        }

        write!(f, "(Frame: {}, Thread: {}, Time: {}, Size: {}, Channels: {}, Bits/sample: (1){}, Real: {}, Valid: {}, Station: {} ({}))",
        self.frameno, self.thread, self.time, self.size, self.channels, self.bits_per_sample, self.is_real, self.is_valid, station, self.station)
    }
}