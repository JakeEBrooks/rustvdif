//! Functions for encoding VDIF headers

use crate::frame::VDIFFrame;
use crate::header_masks::*;

/// Encode the 'Invalid data' header field into a [`VDIFFrame`].
pub fn encode_is_valid(frame: &mut VDIFFrame, is_valid: bool) {
    if is_valid {
        frame.as_mut_slice()[0] &= !MASK_IS_VALID
    } else {
        frame.as_mut_slice()[0] |= MASK_IS_VALID
    }
}

/// Encode the 'Legacy mode' header field into a [`VDIFFrame`].
pub fn encode_is_legacy(frame: &mut VDIFFrame, is_legacy: bool) {
    if is_legacy {
        frame.as_mut_slice()[0] |= MASK_IS_LEGACY
    } else {
        frame.as_mut_slice()[0] &= !MASK_IS_LEGACY
    }
}

/// Encode the 'Seconds from reference epoch' header field into a [`VDIFFrame`].
pub fn encode_time(frame: &mut VDIFFrame, time: u32) {
    frame.as_mut_slice()[0] |= time & MASK_TIME
}

/// Encode the 'Reference Epoch' header field into a [`VDIFFrame`].
pub fn encode_ref_epoch(frame: &mut VDIFFrame, ref_epoch: u8) {
    frame.as_mut_slice()[1] |= ((ref_epoch as u32) << 24) & MASK_REF_EPOCH
}

/// Encode the 'Data Frame number within second' header field into a [`VDIFFrame`].
pub fn encode_frameno(frame: &mut VDIFFrame, frameno: u32) {
    frame.as_mut_slice()[1] |= frameno & MASK_FRAME_NO
}

/// Encode the 'VDIF version number' header field into a [`VDIFFrame`].
pub fn encode_version(frame: &mut VDIFFrame, version: u8) {
    frame.as_mut_slice()[2] |= ((version as u32) << 29) & MASK_VERSION_NO
}

/// Encode the log<sub>2</sub>(channelno) header field into a [`VDIFFrame`].
pub fn encode_log2channels(frame: &mut VDIFFrame, log2channels: u8) {
    frame.as_mut_slice()[2] |= ((log2channels as u32) << 24) & MASK_LOG2_CHANNELS
}

/// Encode the 'Data Frame length' header field into a [`VDIFFrame`].
///
/// Note this is the size of the data frame in **units of eight bytes**.
pub fn encode_size8(frame: &mut VDIFFrame, size8: u32) {
    frame.as_mut_slice()[2] |= size8 & MASK_SIZE8
}

/// Encode the 'Data type' header field into a [`VDIFFrame`].
pub fn encode_is_real(frame: &mut VDIFFrame, is_real: bool) {
    if is_real {
        frame.as_mut_slice()[3] &= !MASK_IS_REAL
    } else {
        frame.as_mut_slice()[3] |= MASK_IS_REAL
    }
}

/// Encode the 'bits per sample' header field into a [`VDIFFrame`].
///
/// This is the bit precision of each sample **minus one**.
pub fn encode_bits_per_sample_1(frame: &mut VDIFFrame, bits_per_sample_1: u8) {
    frame.as_mut_slice()[3] |= ((bits_per_sample_1 as u32) << 26) & MASK_BITS_PER_SAMPLE
}

/// Encode the 'Thread ID' header field into a [`VDIFFrame`].
pub fn encode_threadid(frame: &mut VDIFFrame, threadid: u16) {
    frame.as_mut_slice()[3] |= ((threadid as u32) << 16) & MASK_THREAD_ID
}

/// Encode the 'Station ID' header field into a [`VDIFFrame`].
pub fn encode_stationid(frame: &mut VDIFFrame, stationid: u16) {
    frame.as_mut_slice()[3] |= (stationid as u32) & MASK_STATION_ID
}