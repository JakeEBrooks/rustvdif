use crate::{decoding::header::*, header_masks::*, VDIFHeader};

// It would be kind of insane to create a 8MB frame, so if the user tried to do this
// something has probably gone terribly wrong 
const MAX_FRAME_SIZE: u32 = 8000000;

/// A VDIF Frame.
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
            data.len() % 2 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self { data };
    }

    /// Construct a [`VDIFFrame`] by copying the contents of `data`.
    pub fn from_slice(data: &[u32]) -> Self {
        assert!(
            data.len() % 2 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self {
            data: Box::from(data),
        };
    }

    /// Construct a [`VDIFFrame`] by copying the contents of a `&[u8]` byte slice.
    pub fn from_byte_slice(data: &[u8]) -> Self {
        assert!(
            data.len() % 8 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self { data: Box::from(
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) }
        ) }
    }

    /// Construct a completely empty [`VDIFFrame`] with all header and data bytes set to zero.
    pub fn new_empty(frame_size: usize) -> Self {
        assert!(
            frame_size % 8 == 0,
            "VDIF frames must be a multiple of 8 bytes in size."
        );
        return Self {
            data: vec![0; frame_size / 4].into_boxed_slice(),
        };
    }

    /// Construct a completely empty [`VDIFFrame`] with the invalid bit set, and all other bits set to zero.
    pub fn new_invalid(frame_size: usize) -> Self {
        let mut out = Self::new_empty(frame_size);
        out.as_mut_slice()[0] |= MASK_IS_VALID;
        return out;
    }

    /// Construct a [`VDIFHeader`] by copying the data from the header part of `self`
    pub fn get_header(&self) -> VDIFHeader {
        return VDIFHeader::from_slice(self.data[0..8].try_into().unwrap())
    }

    /// Construct a [`VDIFFrame`] by copying the information contained in a [`VDIFHeader`].
    pub fn from_header(header: VDIFHeader) -> Self {
        let size = header.get_size8() * 8;
        debug_assert!(size < MAX_FRAME_SIZE, "Tried to create a VDIF frame larger than 8MB!");
        let mut frame = Self::new_empty(size as usize);
        frame.as_mut_slice()[0..8].copy_from_slice(header.as_slice());
        return frame
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

    /// Return true if `self` contains zero bytes.
    pub fn is_empty(&self) -> bool {
        return self.len() == 0
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

    /// Return an unsafe pointer to the underlying data.
    pub const fn as_ptr(&self) -> *const u32 {
        return self.data.as_ptr()
    }

    /// Return an unsafe mutable pointer to the underlying data.
    pub const fn as_mut_ptr(&mut self) -> *mut u32 {
        return self.data.as_mut_ptr()
    }

    /// Set the 'Invalid data' field.
    pub fn set_valid(&mut self, is_valid: bool) {
        if is_valid {
            self.data[0] &= !MASK_IS_VALID
        } else {
            self.data[0] |= MASK_IS_VALID
        }
    }

    /// Set the 'Legacy mode' field.
    pub fn set_legacy(&mut self, is_legacy: bool) {
        if is_legacy {
            self.data[0] |= MASK_IS_LEGACY
        } else {
            self.data[0] &= !MASK_IS_LEGACY
        }
    }

    /// Set the 'Seconds from reference epoch' field.
    pub fn set_time(&mut self, time: u32) {
        self.data[0] |= time & MASK_TIME;
    }

    /// Set the 'Reference epoch for second count' field.
    pub fn set_ref_epoch(&mut self, ref_epoch: u8) {
        self.data[1] |= ((ref_epoch as u32) << 24) & MASK_REF_EPOCH;
    }

    /// Set the 'Data frame # within second' field.
    pub fn set_frameno(&mut self, frameno: u32) {
        self.data[1] |= frameno & MASK_FRAME_NO;
    }

    /// Set the 'VDIF version number' field.
    pub fn set_version(&mut self, version: u8) {
        self.data[2] |= ((version as u32) << 29) & MASK_VERSION_NO;
    }

    /// Set the 'log<sub>2</sub>(channelno)' field.
    /// 
    /// For example if you have 4 channels in your payload data, then pass 2 to this function.
    pub fn set_log2channels(&mut self, log2channels: u8) {
        self.data[2] |= ((log2channels as u32) << 24) & MASK_LOG2_CHANNELS;
    }

    /// Set the 'Data frame length' field.
    ///
    /// Note this is the size of the data frame in **units of eight bytes**.
    pub fn set_size8(&mut self, size8: u32) {
        self.data[2] |= size8 & MASK_SIZE8;
    }

    /// Set the 'Data type' field.
    pub fn set_real(&mut self, is_real: bool) {
        if is_real {
            self.data[3] &= !MASK_IS_REAL
        } else {
            self.data[3] |= MASK_IS_REAL
        }
    }

    /// Set the 'bits per sample' field.
    /// 
    /// This is the bit precision of each sample **minus one**.
    pub fn set_bits_per_sample_1(&mut self, bits_per_sample_1: u8) {
        self.data[3] |= ((bits_per_sample_1 as u32) << 26) & MASK_BITS_PER_SAMPLE;
    }

    /// Set the 'Thread ID' field.
    pub fn set_thread(&mut self, threadid: u16) {
        self.data[3] |= ((threadid as u32) << 16) & MASK_THREAD_ID;
    }

    /// Set the 'Station ID' field.
    pub fn set_station(&mut self, stationid: u16) {
        self.data[3] |= (stationid as u32) & MASK_STATION_ID;
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

impl std::fmt::Display for VDIFFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "<Valid: {}, Time: {}, Epoch: {}, Frame: {}, Chans: {}, Size: {}, Real: {}, Bits per sample: {}, Thread: {}, Station: {}>",
        self.get_valid(),
        self.get_time(),
        self.get_ref_epoch(),
        self.get_frameno(),
        1u8 << self.get_log2channels(),
        self.get_size8()*8,
        self.get_real(),
        self.get_bits_per_sample_1()+1,
        self.get_thread(),
        self.get_station())
    }
}