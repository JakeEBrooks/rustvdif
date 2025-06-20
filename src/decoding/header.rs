//! Functions for decoding VDIF headers

use crate::header_masks::*;

/// Decode the 'Invalid data' header field from a VDIF `u32` word.
pub fn decode_is_valid(word: u32) -> bool {
    return (word & MASK_IS_VALID) == 0;
}

/// Decode the 'Legacy mode' header field from a VDIF `u32` word.
pub fn decode_is_legacy(word: u32) -> bool {
    return (word & MASK_IS_LEGACY) != 0;
}

/// Decode the 'Seconds from reference epoch' header field from a VDIF `u32` word.
pub fn decode_time(word: u32) -> u32 {
    return word & MASK_TIME;
}

/// Decode the 'Reference Epoch' header field from a VDIF `u32` word.
pub fn decode_ref_epoch(word: u32) -> u8 {
    return ((word & MASK_REF_EPOCH) >> 24) as u8;
}

/// Decode the 'Data Frame number within second' header field from a VDIF `u32` word.
pub fn decode_frameno(word: u32) -> u32 {
    return word & MASK_FRAME_NO;
}

/// Decode the 'VDIF version number' header field from a VDIF `u32` word.
pub fn decode_version(word: u32) -> u8 {
    return ((word & MASK_VERSION_NO) >> 29) as u8;
}

/// Decode the log<sub>2</sub>(channelno) header field from a VDIF `u32` word.
pub fn decode_log2channels(word: u32) -> u8 {
    return ((word & MASK_LOG2_CHANNELS) >> 24) as u8;
}

/// Decode the 'Data Frame length' header field from a VDIF `u32` word.
///
/// Note this is the size of the data frame in **units of eight bytes**.
pub fn decode_size8(word: u32) -> u32 {
    return word & MASK_SIZE8;
}

/// Decode the 'Data type' header field from a VDIF `u32` word.
pub fn decode_is_real(word: u32) -> bool {
    return (word & MASK_IS_REAL) == 0;
}

/// Decode the 'bits per sample' header field from a VDIF `u32` word.
///
/// This is the bit precision of each sample **minus one**.
pub fn decode_bits_per_sample_1(word: u32) -> u8 {
    return ((word & MASK_BITS_PER_SAMPLE) >> 26) as u8;
}

/// Decode the 'Thread ID' header field from a VDIF `u32` word.
pub fn decode_threadid(word: u32) -> u16 {
    return ((word & MASK_THREAD_ID) >> 16) as u16;
}

/// Decode the 'Station ID' header field from a VDIF `u32` word.
pub fn decode_stationid(word: u32) -> u16 {
    return (word & MASK_STATION_ID) as u16;
}
