use crate::frame::VDIFFrame;
use crate::header::VDIFHeader;

pub(crate) const MASK_IS_VALID: u32 = 0b10000000000000000000000000000000;
pub(crate) const MASK_IS_LEGACY: u32 = 0b01000000000000000000000000000000;
pub(crate) const MASK_TIME: u32 = 0b00111111111111111111111111111111;
pub(crate) const MASK_REF_EPOCH: u32 = 0b00111111000000000000000000000000;
pub(crate) const MASK_FRAME_NO: u32 = 0b00000000111111111111111111111111;
pub(crate) const MASK_VERSION_NO: u32 = 0b11100000000000000000000000000000;
pub(crate) const MASK_LOG2_CHANNELS: u32 = 0b00011111000000000000000000000000;
pub(crate) const MASK_BYTE_SIZE: u32 = 0b00000000111111111111111111111111;
pub(crate) const MASK_IS_REAL: u32 = 0b10000000000000000000000000000000;
pub(crate) const MASK_BITS_PER_SAMPLE: u32 = 0b01111100000000000000000000000000;
pub(crate) const MASK_THREAD_ID: u32 = 0b00000011111111110000000000000000;
pub(crate) const MASK_STATION_ID: u32 = 0b00000000000000001111111111111111;

/// Decode the [`VDIFHeader`] of a [`VDIFFrame`].
pub fn decode_frame_header(frame: &VDIFFrame) -> VDIFHeader {
    return decode_header(frame.as_slice()[0..8].try_into().unwrap());
}

/// Decode a [`VDIFHeader`] from an array of eight `u32`s.
pub fn decode_header(words: [u32; 8]) -> VDIFHeader {
    let (is_valid, is_legacy, time) = decode_w0(words[0]);
    let (epoch, frameno) = decode_w1(words[1]);
    let (version, channels, size) = decode_w2(words[2]);
    let (is_real, bits_per_sample, thread, station) = decode_w3(words[3]);
    let edv0 = words[4];
    let edv1 = words[5];
    let edv2 = words[6];
    let edv3 = words[7];

    return VDIFHeader {
        is_valid: is_valid,
        is_legacy: is_legacy,
        time: time,
        epoch: epoch,
        frameno: frameno,
        version: version,
        channels: channels,
        size: size,
        is_real: is_real,
        bits_per_sample: bits_per_sample,
        thread: thread,
        station: station,
        edv0: edv0,
        edv1: edv1,
        edv2: edv2,
        edv3: edv3,
    };
}

/// Decode the zeroth word of a VDIFHeader
pub(crate) fn decode_w0(word: u32) -> (bool, bool, u32) {
    let is_valid = (word & MASK_IS_VALID) == 0;
    let is_legacy = (word & MASK_IS_LEGACY) != 0;
    let time = word & MASK_TIME;
    return (is_valid, is_legacy, time);
}

/// Decode the first word of a VDIFHeader
pub(crate) fn decode_w1(word: u32) -> (u8, u32) {
    let epoch = ((word & MASK_REF_EPOCH) >> 24) as u8;
    let frameno = word & MASK_FRAME_NO;
    return (epoch, frameno);
}

/// Decode the second word of a VDIFHeader
pub(crate) fn decode_w2(word: u32) -> (u8, u8, u32) {
    let version = ((word & MASK_VERSION_NO) >> 29) as u8;
    let channels = ((word & MASK_LOG2_CHANNELS) >> 24) as u8;
    let size = word & MASK_BYTE_SIZE;
    return (version, channels, size);
}

/// Decode the third word of a VDIFHeader
pub(crate) fn decode_w3(word: u32) -> (bool, u8, u16, u16) {
    let is_real = (word & MASK_IS_REAL) == 0;
    let bits_per_sample = ((word & MASK_BITS_PER_SAMPLE) >> 26) as u8;
    let thread = ((word & MASK_THREAD_ID) >> 16) as u16;
    let station = (word & MASK_STATION_ID) as u16;
    return (is_real, bits_per_sample, thread, station);
}

/// Encode a [`VDIFHeader`] into an array of eight `u32`s.
pub fn encode_header(header: VDIFHeader) -> [u32; 8] {
    let mut w0 = header.time;
    if header.is_legacy {
        w0 = w0 | MASK_IS_LEGACY
    } else {
        w0 = w0 & (!MASK_IS_LEGACY)
    }
    if header.is_valid {
        w0 = w0 & (!MASK_IS_VALID)
    } else {
        w0 = w0 | MASK_IS_VALID
    }

    let w1 = header.frameno | ((header.epoch as u32) << 24);
    let w2 = header.size | ((header.channels as u32) << 24) | ((header.version as u32) << 29);
    let mut w3 = header.station as u32
        | ((header.thread as u32) << 16)
        | ((header.bits_per_sample as u32) << 26);
    if header.is_real {
        w3 = w3 & (!MASK_IS_REAL)
    } else {
        w3 = w3 | MASK_IS_REAL
    }

    let w4 = header.edv0;
    let w5 = header.edv1;
    let w6 = header.edv2;
    let w7 = header.edv3;

    return [w0, w1, w2, w3, w4, w5, w6, w7];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::*;

    #[test]
    fn test_header_encoding() {
        let test_header = VDIFHeader {
            is_valid: true,
            is_legacy: false,
            time: 40,
            epoch: 2,
            frameno: 1072,
            version: 0,
            channels: 2,
            size: 8032,
            is_real: true,
            bits_per_sample: 4,
            thread: 0,
            station: 124,

            edv0: 0,
            edv1: 0,
            edv2: 0,
            edv3: 0,
        };

        let cpy = test_header;
        assert_eq!(cpy, decode_header(encode_header(test_header)))
    }
}
