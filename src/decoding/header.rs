//! Functions for decoding VDIF headers

use crate::frame::VDIFFrame;
use crate::header_masks::*;

/// Decode the 'Invalid data' header field from a [`VDIFFrame`].
pub fn decode_is_valid(frame: &VDIFFrame) -> bool {
    return (frame.as_slice()[0] & MASK_IS_VALID) == 0;
}

/// Decode the 'Legacy mode' header field from a [`VDIFFrame`].
pub fn decode_is_legacy(frame: &VDIFFrame) -> bool {
    return (frame.as_slice()[0] & MASK_IS_LEGACY) != 0;
}

/// Decode the 'Seconds from reference epoch' header field from a [`VDIFFrame`].
pub fn decode_time(frame: &VDIFFrame) -> u32 {
    return frame.as_slice()[0] & MASK_TIME;
}

/// Decode the 'Reference Epoch' header field from a [`VDIFFrame`].
pub fn decode_ref_epoch(frame: &VDIFFrame) -> u8 {
    return ((frame.as_slice()[1] & MASK_REF_EPOCH) >> 24) as u8;
}

/// Decode the 'Data Frame number within second' header field from a [`VDIFFrame`].
pub fn decode_frameno(frame: &VDIFFrame) -> u32 {
    return frame.as_slice()[1] & MASK_FRAME_NO;
}

/// Decode the 'VDIF version number' header field from a [`VDIFFrame`].
pub fn decode_version(frame: &VDIFFrame) -> u8 {
    return ((frame.as_slice()[2] & MASK_VERSION_NO) >> 29) as u8;
}

/// Decode the log<sub>2</sub>(channelno) header field from a [`VDIFFrame`].
pub fn decode_log2channels(frame: &VDIFFrame) -> u8 {
    return ((frame.as_slice()[2] & MASK_LOG2_CHANNELS) >> 24) as u8;
}

/// Decode the 'Data Frame length' header field from a [`VDIFFrame`].
///
/// Note this is the size of the data frame in **units of eight bytes**.
pub fn decode_size8(frame: &VDIFFrame) -> u32 {
    return frame.as_slice()[2] & MASK_SIZE8;
}

/// Decode the 'Data type' header field from a [`VDIFFrame`].
pub fn decode_is_real(frame: &VDIFFrame) -> bool {
    return (frame.as_slice()[3] & MASK_IS_REAL) == 0;
}

/// Decode the 'bits per sample' header field from a [`VDIFFrame`].
///
/// This is the bit precision of each sample **minus one**.
pub fn decode_bits_per_sample_1(frame: &VDIFFrame) -> u8 {
    return ((frame.as_slice()[3] & MASK_BITS_PER_SAMPLE) >> 26) as u8;
}

/// Decode the 'Thread ID' header field from a [`VDIFFrame`].
pub fn decode_threadid(frame: &VDIFFrame) -> u16 {
    return ((frame.as_slice()[3] & MASK_THREAD_ID) >> 16) as u16;
}

/// Decode the 'Station ID' header field from a [`VDIFFrame`].
pub fn decode_stationid(frame: &VDIFFrame) -> u16 {
    return (frame.as_slice()[3] & MASK_STATION_ID) as u16;
}
