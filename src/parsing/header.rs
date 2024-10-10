//! Provides functionality for parsing byte slices into [`VDIFHeader`]s.

use nom::IResult;
use nom::number::complete::le_u32;

use crate::header::VDIFHeader;

const MASK_IS_VALID:        u32 = 0b10000000000000000000000000000000;
const MASK_IS_LEGACY:       u32 = 0b01000000000000000000000000000000;
const MASK_TIME:            u32 = 0b00111111111111111111111111111111;
const MASK_REF_EPOCH:       u32 = 0b00111111000000000000000000000000;
const MASK_FRAME_NO:        u32 = 0b00000000111111111111111111111111;
const MASK_VERSION_NO:      u32 = 0b11100000000000000000000000000000;
const MASK_LOG2_CHANNELS:   u32 = 0b00011111000000000000000000000000;
const MASK_BYTE_SIZE:       u32 = 0b00000000111111111111111111111111;
const MASK_IS_REAL:         u32 = 0b10000000000000000000000000000000;
const MASK_BITS_PER_SAMPLE: u32 = 0b01111100000000000000000000000000;
const MASK_THREAD_ID:       u32 = 0b00000011111111110000000000000000;
const MASK_STATION_ID:      u32 = 0b00000000000000001111111111111111;

/// Parse a [`VDIFHeader`] from a byte slice.
pub fn parse_header(input: &[u8]) -> IResult<&[u8], VDIFHeader> {
    let (remaining, (is_valid, is_legacy, time)) = parse_w0(input)?;
    let (remaining, (epoch, frame)) = parse_w1(remaining)?;
    let (remaining, (version, channels, size)) = parse_w2(remaining)?;
    let (remaining, (is_real, bits_per_sample, thread, station)) = parse_w3(remaining)?;
    let (remaining, edv0) = le_u32(remaining)?;
    let (remaining, edv1) = le_u32(remaining)?;
    let (remaining, edv2) = le_u32(remaining)?;
    let (remaining, edv3) = le_u32(remaining)?;
    return Ok((remaining, VDIFHeader{is_valid: is_valid, is_legacy: is_legacy, time: time,
                epoch: epoch, frame: frame, version: version, channels: channels, size: size,
                is_real: is_real, bits_per_sample: bits_per_sample, thread: thread, station: station,
                    edv0: edv0,
                    edv1: edv1,
                    edv2: edv2,
                    edv3: edv3
                }))
}

fn parse_w0(input: &[u8]) -> IResult<&[u8], (bool, bool, u32)> {
    let (remaining, word) = le_u32(input)?;
    let is_valid = (word & MASK_IS_VALID) == 0;
    let is_legacy = (word & MASK_IS_LEGACY) != 0;
    let time = word & MASK_TIME;
    return Ok((remaining, (is_valid, is_legacy, time)))
}

fn parse_w1(input: &[u8]) -> IResult<&[u8], (u8, u32)> {
    let (remaining, word) = le_u32(input)?;
    let epoch = ((word & MASK_REF_EPOCH) >> 24) as u8;
    let frame = word & MASK_FRAME_NO;
    return Ok((remaining, (epoch, frame)))
}

fn parse_w2(input: &[u8]) -> IResult<&[u8], (u8, u8, u32)> {
    let (remaining, word) = le_u32(input)?;
    let version = ((word & MASK_VERSION_NO) >> 29) as u8;
    let channels = ((word & MASK_LOG2_CHANNELS) >> 24) as u8;
    let size = (word & MASK_BYTE_SIZE) * 8;
    return Ok((remaining, (version, channels, size)))
}

fn parse_w3(input: &[u8]) -> IResult<&[u8], (bool, u8, u16, u16)> {
    let (remaining, word) = le_u32(input)?;
    let is_real = (word & MASK_IS_REAL) == 0;
    let bits_per_sample = ((word & MASK_BITS_PER_SAMPLE) >> 26) as u8;
    let thread = ((word & MASK_THREAD_ID) >> 16) as u16;
    let station = (word & MASK_STATION_ID) as u16;
    return Ok((remaining, (is_real, bits_per_sample, thread, station)))
}
