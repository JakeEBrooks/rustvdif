use std::mem::transmute;

use crate::{header_masks::*, decoding::header::*};

/// A VDIF Header.
/// 
/// A [`VDIFHeader`] can be easily constructed using the following pattern:
/// ```
/// use rustvdif::VDIFHeader;
/// let my_header = VDIFHeader::new()
///     .time(1234)
///     .ref_epoch(6)
///     .size8(1004);
/// assert_eq!(my_header.as_slice()[0] & 0x3FFFFFFF, 1234)
/// ```
/// to create an empty [`VDIFHeader`] with the time field set to 1234, the reference epoch set to
/// 6, and the frame size field set to 1004 (i.e. 8032 bytes).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VDIFHeader {
    data: [u32; 8]
}

impl VDIFHeader {
    /// Construct a [`VDIFHeader`] with all fields set to zero.
    pub fn new() -> Self {
        return Self::default()
    }

    /// Construct an empty [`VDIFHeader`] and set the invalid bit.
    pub fn new_invalid() -> Self {
        let mut hdr = Self::new();
        hdr.data[0] |= MASK_IS_VALID;
        return hdr
    }

    /// Construct a [`VDIFHeader`] from an array of `u32` words.
    pub fn from_slice(data: [u32; 8]) -> Self {
        return Self { data: data }
    }

    /// Construct a [`VDIFHeader`] from an array of bytes.
    pub fn from_bytes(data: [u8; 32]) -> Self {
        return Self { data: unsafe { transmute(data) } }
    }

    /// Get a reference to the underlying data.
    pub fn as_slice(&self) -> &[u32; 8] {
        return &self.data
    }

    /// Get a mutable reference to underlying data.
    pub fn as_mut_slice(&mut self) -> &mut [u32; 8] {
        return &mut self.data
    }

    /// Set the 'Invalid data' field.
    pub fn valid(mut self, is_valid: bool) -> Self {
        if is_valid {
            self.data[0] &= !MASK_IS_VALID
        } else {
            self.data[0] |= MASK_IS_VALID
        }

        return self
    }

    /// Set the 'Legacy mode' field.
    pub fn legacy(mut self, is_legacy: bool) -> Self {
        if is_legacy {
            self.data[0] |= MASK_IS_LEGACY
        } else {
            self.data[0] &= !MASK_IS_LEGACY
        }

        return self
    }

    /// Set the 'Seconds from reference epoch' field.
    pub fn time(mut self, time: u32) -> Self {
        self.data[0] |= time & MASK_TIME;
        return self
    }

    /// Set the 'Reference epoch for second count' field.
    pub fn ref_epoch(mut self, ref_epoch: u8) -> Self {
        self.data[1] |= ((ref_epoch as u32) << 24) & MASK_REF_EPOCH;
        return self
    }

    /// Set the 'Data frame # within second' field.
    pub fn frameno(mut self, frameno: u32) -> Self {
        self.data[1] |= frameno & MASK_FRAME_NO;
        return self
    }

    /// Set the 'VDIF version number' field.
    pub fn version(mut self, version: u8) -> Self {
        self.data[2] |= ((version as u32) << 29) & MASK_VERSION_NO;
        return self
    }

    /// Set the 'log<sub>2</sub>(channelno)' field.
    /// 
    /// For example if you have 4 channels in your payload data, then pass 2 to this function.
    pub fn log2channels(mut self, log2channels: u8) -> Self {
        self.data[2] |= ((log2channels as u32) << 24) & MASK_LOG2_CHANNELS;
        return self
    }

    /// Set the 'Data frame length' field.
    ///
    /// Note this is the size of the data frame in **units of eight bytes**.
    pub fn size8(mut self, size8: u32) -> Self {
        self.data[2] |= size8 & MASK_SIZE8;
        return self
    }

    /// Set the 'Data type' field.
    pub fn real(mut self, is_real: bool) -> Self {
        if is_real {
            self.data[3] &= !MASK_IS_REAL
        } else {
            self.data[3] |= MASK_IS_REAL
        }
        return self
    }

    /// Set the 'bits per sample' field.
    /// 
    /// This is the bit precision of each sample **minus one**.
    pub fn bits_per_sample_1(mut self, bits_per_sample_1: u8) -> Self {
        self.data[3] |= ((bits_per_sample_1 as u32) << 26) & MASK_BITS_PER_SAMPLE;
        return self
    }

    /// Set the 'Thread ID' field.
    pub fn thread(mut self, threadid: u16) -> Self {
        self.data[3] |= ((threadid as u32) << 16) & MASK_THREAD_ID;
        return self
    }

    /// Set the 'Station ID' field.
    pub fn station(mut self, stationid: u16) -> Self {
        self.data[3] |= (stationid as u32) & MASK_STATION_ID;
        return self
    }

    /// Get the 'Invalid data' field.
    pub fn get_valid(&self) -> bool {
        return decode_is_valid(self.data[0])
    }

    /// Get the 'Legacy mode' field.
    pub fn get_legacy(&self) -> bool {
        return decode_is_legacy(self.data[0])
    }

    /// Get the 'Seconds from reference epoch' field.
    pub fn get_time(&self) -> u32 {
        return decode_time(self.data[0])
    }

    /// Get the 'Reference epoch for second count' field.
    pub fn get_ref_epoch(&self) -> u8 {
        return decode_ref_epoch(self.data[1])
    }

    /// Get the 'Data frame # within second' field.
    pub fn get_frameno(&self) -> u32 {
        return decode_frameno(self.data[1])
    }

    /// Get the 'VDIF version number' field.
    pub fn get_version(&self) -> u8 {
        return decode_version(self.data[2])
    }

    /// Get the 'log<sub>2</sub>(channelno)' field.
    /// 
    /// For example if you have 4 channels in your payload data, this will return 2.
    pub fn get_log2channels(&self) -> u8 {
        return decode_log2channels(self.data[2])
    }

    /// Get the 'Data frame length' field.
    ///
    /// Note this is the size of the data frame in **units of eight bytes**.
    pub fn get_size8(&self) -> u32 {
        return decode_size8(self.data[2])
    }

    /// Get the 'Data type' field.
    pub fn get_real(&self) -> bool {
        return decode_is_real(self.data[3])
    }

    /// Get the 'bits per sample' field.
    /// 
    /// This is the bit precision of each sample **minus one**.
    pub fn get_bits_per_sample_1(&self) -> u8 {
        return decode_bits_per_sample_1(self.data[3])
    }

    /// Get the 'Thread ID' field.
    pub fn get_thread(&self) -> u16 {
        return decode_threadid(self.data[3])
    }

    /// Get the 'Station ID' field.
    pub fn get_station(&self) -> u16 {
        return decode_stationid(self.data[3])
    }
}