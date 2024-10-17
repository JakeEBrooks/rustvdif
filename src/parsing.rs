//! Provides functionality for parsing byte slices into various VDIF types.
//!
//! The parsing is done using [`nom`] and all the parsing functions are [`nom`] parsers,
//! so see their documentation to understand these functions.

use nom::number::complete::le_u32;
use nom::IResult;

use crate::frame::VDIFFrame;
use crate::header::VDIFHeader;
use crate::payload::VDIFPayload;

pub(crate) const VDIF_HEADER_SIZE: usize = 8;
pub(crate) const VDIF_HEADER_BYTESIZE: usize = 32;

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
    return Ok((
        remaining,
        VDIFHeader {
            is_valid: is_valid,
            is_legacy: is_legacy,
            time: time,
            epoch: epoch,
            frame: frame,
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
        },
    ));
}

/// Parse the zeroth word of a VDIFHeader
fn parse_w0(input: &[u8]) -> IResult<&[u8], (bool, bool, u32)> {
    let (remaining, word) = le_u32(input)?;
    let is_valid = (word & MASK_IS_VALID) == 0;
    let is_legacy = (word & MASK_IS_LEGACY) != 0;
    let time = word & MASK_TIME;
    return Ok((remaining, (is_valid, is_legacy, time)));
}

/// Parse the first word of a VDIFHeader
fn parse_w1(input: &[u8]) -> IResult<&[u8], (u8, u32)> {
    let (remaining, word) = le_u32(input)?;
    let epoch = ((word & MASK_REF_EPOCH) >> 24) as u8;
    let frame = word & MASK_FRAME_NO;
    return Ok((remaining, (epoch, frame)));
}

/// Parse the second word of a VDIFHeader
fn parse_w2(input: &[u8]) -> IResult<&[u8], (u8, u8, u32)> {
    let (remaining, word) = le_u32(input)?;
    let version = ((word & MASK_VERSION_NO) >> 29) as u8;
    let channels = ((word & MASK_LOG2_CHANNELS) >> 24) as u8;
    let size = (word & MASK_BYTE_SIZE) * 8;
    return Ok((remaining, (version, channels, size)));
}

/// Parse the third word of a VDIFHeader
fn parse_w3(input: &[u8]) -> IResult<&[u8], (bool, u8, u16, u16)> {
    let (remaining, word) = le_u32(input)?;
    let is_real = (word & MASK_IS_REAL) == 0;
    let bits_per_sample = ((word & MASK_BITS_PER_SAMPLE) >> 26) as u8;
    let thread = ((word & MASK_THREAD_ID) >> 16) as u16;
    let station = (word & MASK_STATION_ID) as u16;
    return Ok((remaining, (is_real, bits_per_sample, thread, station)));
}

/// Parse a [`VDIFPayload`] from a byte slice. Requires a reference to the associated [`VDIFHeader`]
/// to ensure the correct number of bytes are parsed.
pub fn parse_payload<'a, 'b>(
    input: &'a [u8],
    header: &'b VDIFHeader,
) -> IResult<&'a [u8], VDIFPayload> {
    let payload_wordsize = header.payload_wordsize();
    let mut out: Vec<u32> = Vec::with_capacity(payload_wordsize as usize);
    let (mut remaining, mut word) = le_u32(input)?;
    out.push(word);
    for _ in 1..payload_wordsize {
        (remaining, word) = le_u32(remaining)?;
        out.push(word);
    }

    return Ok((remaining, VDIFPayload::new(out.into_boxed_slice())));
}

/// Parse a [`VDIFFrame`] from a byte slice.
pub fn parse_frame(input: &[u8]) -> IResult<&[u8], VDIFFrame> {
    let (remaining, header) = parse_header(input)?;
    let (remaining, payload) = parse_payload(remaining, &header)?;
    return Ok((remaining, VDIFFrame::new(header, payload)));
}

/// Parse a specific number of [`VDIFFrame`]s from the input byte slice using [`nom::multi::count`].
pub fn parse_frames(input: &[u8], numframes: usize) -> IResult<&[u8], Vec<VDIFFrame>> {
    return nom::multi::count(parse_frame, numframes)(input);
}

/// Parse as many frames as possible from the input using [`nom::multi::many0`].
pub fn parse_all_frames(input: &[u8]) -> IResult<&[u8], Vec<VDIFFrame>> {
    return nom::multi::many0(parse_frame)(input);
}
