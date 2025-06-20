//! Functions for encoding VDIF headers

use crate::header_masks::*;

/// Encode the 'Invalid data' header field into a VDIF `u32` word.
pub fn encode_is_valid(word: &mut u32, is_valid: bool) {
    if is_valid {
        *word &= !MASK_IS_VALID
    } else {
        *word |= MASK_IS_VALID
    }
}

/// Encode the 'Legacy mode' header field into a VDIF `u32` word.
pub fn encode_is_legacy(word: &mut u32, is_legacy: bool) {
    if is_legacy {
        *word |= MASK_IS_LEGACY
    } else {
        *word &= !MASK_IS_LEGACY
    }
}

/// Encode the 'Seconds from reference epoch' header field into a VDIF `u32` word.
pub fn encode_time(word: &mut u32, time: u32) {
    *word |= time & MASK_TIME
}

/// Encode the 'Reference Epoch' header field into a VDIF `u32` word.
pub fn encode_ref_epoch(word: &mut u32, ref_epoch: u8) {
    *word |= ((ref_epoch as u32) << 24) & MASK_REF_EPOCH
}

/// Encode the 'Data Frame number within second' header field into a VDIF `u32` word.
pub fn encode_frameno(word: &mut u32, frameno: u32) {
    *word |= frameno & MASK_FRAME_NO
}

/// Encode the 'VDIF version number' header field into a VDIF `u32` word.
pub fn encode_version(word: &mut u32, version: u8) {
    *word |= ((version as u32) << 29) & MASK_VERSION_NO
}

/// Encode the log<sub>2</sub>(channelno) header field into a VDIF `u32` word.
pub fn encode_log2channels(word: &mut u32, log2channels: u8) {
    *word |= ((log2channels as u32) << 24) & MASK_LOG2_CHANNELS
}

/// Encode the 'Data Frame length' header field into a VDIF `u32` word.
///
/// Note this is the size of the data frame in **units of eight bytes**.
pub fn encode_size8(word: &mut u32, size8: u32) {
    *word |= size8 & MASK_SIZE8
}

/// Encode the 'Data type' header field into a VDIF `u32` word.
pub fn encode_is_real(word: &mut u32, is_real: bool) {
    if is_real {
        *word &= !MASK_IS_REAL
    } else {
        *word |= MASK_IS_REAL
    }
}

/// Encode the 'bits per sample' header field into a VDIF `u32` word.
///
/// This is the bit precision of each sample **minus one**.
pub fn encode_bits_per_sample_1(word: &mut u32, bits_per_sample_1: u8) {
    *word |= ((bits_per_sample_1 as u32) << 26) & MASK_BITS_PER_SAMPLE
}

/// Encode the 'Thread ID' header field into a VDIF `u32` word.
pub fn encode_threadid(word: &mut u32, threadid: u16) {
    *word |= ((threadid as u32) << 16) & MASK_THREAD_ID
}

/// Encode the 'Station ID' header field into a VDIF `u32` word.
pub fn encode_stationid(word: &mut u32, stationid: u16) {
    *word |= (stationid as u32) & MASK_STATION_ID
}