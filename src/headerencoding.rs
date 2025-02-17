//! Functions for encoding/decoding VDIF headers

use crate::frame::VDIFFrame;
use crate::header::VDIFHeader;

pub(crate) const MASK_IS_VALID: u32 = 0x80000000;
pub(crate) const MASK_IS_LEGACY: u32 = 0x40000000;
pub(crate) const MASK_TIME: u32 = 0x3FFFFFFF;
pub(crate) const MASK_REF_EPOCH: u32 = 0x3F000000;
pub(crate) const MASK_FRAME_NO: u32 = 0x00FFFFFF;
pub(crate) const MASK_VERSION_NO: u32 = 0xE0000000;
pub(crate) const MASK_LOG2_CHANNELS: u32 = 0x1F000000;
pub(crate) const MASK_SIZE: u32 = 0x00FFFFFF;
pub(crate) const MASK_IS_REAL: u32 = 0x80000000;
pub(crate) const MASK_BITS_PER_SAMPLE: u32 = 0x7C000000;
pub(crate) const MASK_THREAD_ID: u32 = 0x03FF0000;
pub(crate) const MASK_STATION_ID: u32 = 0x0000FFFF;

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
    return frame.as_slice()[2] & MASK_SIZE;
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

/// Decode the [`VDIFHeader`] from a [`VDIFFrame`].
pub fn decode_header(frame: &VDIFFrame) -> VDIFHeader {
    return VDIFHeader {
        is_valid: decode_is_valid(frame),
        is_legacy: decode_is_legacy(frame),
        time: decode_time(frame),
        ref_epoch: decode_ref_epoch(frame),
        frameno: decode_frameno(frame),
        version: decode_version(frame),
        channels: 1u32 << (decode_log2channels(frame) as u32),
        size: decode_size8(frame) * 8,
        is_real: decode_is_real(frame),
        bits_per_sample: decode_bits_per_sample_1(frame) + 1,
        threadid: decode_threadid(frame),
        stationid: decode_stationid(frame),
        edv0: frame.as_slice()[4],
        edv1: frame.as_slice()[5],
        edv2: frame.as_slice()[6],
        edv3: frame.as_slice()[7],
    };
}

/// Encode the fields in the zeroth word of a VDIF header into a `u32`.
pub fn encode_w0(is_valid: bool, is_legacy: bool, time: u32) -> u32 {
    let mut w0 = time;
    if is_valid {
        w0 = w0 & (!MASK_IS_VALID)
    } else {
        w0 = w0 | MASK_IS_VALID
    }
    if is_legacy {
        w0 = w0 | MASK_IS_LEGACY
    } else {
        w0 = w0 & (!MASK_IS_LEGACY)
    }
    return w0;
}

/// Encode the fields in the first word of a VDIF header into a `u32`.
pub fn encode_w1(ref_epoch: u8, frameno: u32) -> u32 {
    return (frameno & MASK_FRAME_NO) | ((ref_epoch as u32) << 24);
}

/// Encode the fields in the second word of a VDIF header into a `u32`.
pub fn encode_w2(version: u8, log2channels: u8, size8: u32) -> u32 {
    return size8 | ((log2channels as u32) << 24) | ((version as u32) << 29);
}

/// Encode the fields in the third word of a VDIF header into a `u32`.
pub fn encode_w3(is_real: bool, bits_per_sample_1: u8, threadid: u16, stationid: u16) -> u32 {
    let mut w3 = stationid as u32 | ((threadid as u32) << 16) | ((bits_per_sample_1 as u32) << 26);
    if is_real {
        w3 = w3 & (!MASK_IS_REAL)
    } else {
        w3 = w3 | MASK_IS_REAL
    }
    return w3;
}

/// Encode a [`VDIFHeader`] into an array of eight `u32`s.
pub fn encode_header(header: &VDIFHeader) -> [u32; 8] {
    return [
        encode_w0(header.is_valid, header.is_legacy, header.time),
        encode_w1(header.ref_epoch, header.frameno),
        encode_w2(
            header.version,
            header.channels.ilog2() as u8,
            header.size / 8,
        ),
        encode_w3(
            header.is_real,
            header.bits_per_sample - 1,
            header.threadid,
            header.stationid,
        ),
        header.edv0,
        header.edv1,
        header.edv2,
        header.edv3,
    ];
}
