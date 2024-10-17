//! Provides functionality for encoding/decoding VDIF headers and payloads.
//!
//! Up to 16-bit encoding is supported, but let me know on GitHub if you have a use case for larger bits/sample.
//!
//! While this crate supports uncommon bits per sample like 6 bit, you should try to stick to 2^n bits per sample
//! (i.e. 1, 2, 4, 8, 16, 32) since they are more efficient to store in VDIF.

use crate::{header::VDIFHeader, parsing::*};

/// Encode a [`VDIFHeader`] into an array of bytes.
pub fn encode_header(header: VDIFHeader) -> [u8; VDIF_HEADER_BYTESIZE] {
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

    let w1 = header.frame | ((header.epoch as u32) << 24);
    let w2 = header.size / 8 | ((header.channels as u32) << 24) | ((header.version as u32) << 29);
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

    let mut out: [u8; VDIF_HEADER_BYTESIZE] = [0; VDIF_HEADER_BYTESIZE];
    let w0bytes = w0.to_le_bytes();
    let w1bytes = w1.to_le_bytes();
    let w2bytes = w2.to_le_bytes();
    let w3bytes = w3.to_le_bytes();
    let w4bytes = w4.to_le_bytes();
    let w5bytes = w5.to_le_bytes();
    let w6bytes = w6.to_le_bytes();
    let w7bytes = w7.to_le_bytes();

    out[0] = w0bytes[0];
    out[1] = w0bytes[1];
    out[2] = w0bytes[2];
    out[3] = w0bytes[3];
    out[4] = w1bytes[0];
    out[5] = w1bytes[1];
    out[6] = w1bytes[2];
    out[7] = w1bytes[3];
    out[8] = w2bytes[0];
    out[9] = w2bytes[1];
    out[10] = w2bytes[2];
    out[11] = w2bytes[3];
    out[12] = w3bytes[0];
    out[13] = w3bytes[1];
    out[14] = w3bytes[2];
    out[15] = w3bytes[3];
    out[16] = w4bytes[0];
    out[17] = w4bytes[1];
    out[18] = w4bytes[2];
    out[19] = w4bytes[3];
    out[20] = w5bytes[0];
    out[21] = w5bytes[1];
    out[22] = w5bytes[2];
    out[23] = w5bytes[3];
    out[24] = w6bytes[0];
    out[25] = w6bytes[1];
    out[26] = w6bytes[2];
    out[27] = w6bytes[3];
    out[28] = w7bytes[0];
    out[29] = w7bytes[1];
    out[30] = w7bytes[2];
    out[31] = w7bytes[3];
    return out;
}

// Other VDIF software uses a LUT for decoding the u32 word, but
// writing it out as below seems to be ~2x faster. This is tested
// for 1 bit decoding, I assume it holds for higher bit depths.

// The decoding functions are quite ugly, but writing it out explicitly
// maximises information to the compiler and doesn't run the risk of
// loops not being unrolled

// The VDIF spec doesn't seem to be clear on how the encoding is applied in cases
// where the number of real samples is not exactly 2x the number of complex samples.
// For example, in 6-bit encoding, you can fit 5 real samples, but only 2 complex samples
// (otherwise a real component would not have an attached complex component).
// In these cases I take the safer approach and maintain the extra real sample.

const MASK_1BIT: u32 = u32::MAX >> 31;
const MASK_2BIT: u32 = u32::MAX >> 30;
const MASK_3BIT: u32 = u32::MAX >> 29;
const MASK_4BIT: u32 = u32::MAX >> 28;

const MASK_6BIT: u32 = u32::MAX >> 26;
const MASK_7BIT: u32 = u32::MAX >> 25;
const MASK_8BIT: u32 = u32::MAX >> 24;

const MASK_11BIT: u32 = u32::MAX >> 21;
const MASK_12BIT: u32 = u32::MAX >> 20;
const MASK_13BIT: u32 = u32::MAX >> 19;
const MASK_14BIT: u32 = u32::MAX >> 18;
const MASK_15BIT: u32 = u32::MAX >> 17;
const MASK_16BIT: u32 = u32::MAX >> 16;

/// Decode a VDIF encoded 32-bit word of 1-bit real samples.
pub fn decode_1bit_real(input: &u32) -> [u8; 32] {
    let mut out: [u8; 32] = [0; 32];

    out[0] = (input & MASK_1BIT) as u8;
    out[1] = ((input >> 1) & MASK_1BIT) as u8;
    out[2] = ((input >> 2) & MASK_1BIT) as u8;
    out[3] = ((input >> 3) & MASK_1BIT) as u8;
    out[4] = ((input >> 4) & MASK_1BIT) as u8;
    out[5] = ((input >> 5) & MASK_1BIT) as u8;
    out[6] = ((input >> 6) & MASK_1BIT) as u8;
    out[7] = ((input >> 7) & MASK_1BIT) as u8;
    out[8] = ((input >> 8) & MASK_1BIT) as u8;
    out[9] = ((input >> 9) & MASK_1BIT) as u8;
    out[10] = ((input >> 10) & MASK_1BIT) as u8;
    out[11] = ((input >> 11) & MASK_1BIT) as u8;
    out[12] = ((input >> 12) & MASK_1BIT) as u8;
    out[13] = ((input >> 13) & MASK_1BIT) as u8;
    out[14] = ((input >> 14) & MASK_1BIT) as u8;
    out[15] = ((input >> 15) & MASK_1BIT) as u8;
    out[16] = ((input >> 16) & MASK_1BIT) as u8;
    out[17] = ((input >> 17) & MASK_1BIT) as u8;
    out[18] = ((input >> 18) & MASK_1BIT) as u8;
    out[19] = ((input >> 19) & MASK_1BIT) as u8;
    out[20] = ((input >> 20) & MASK_1BIT) as u8;
    out[21] = ((input >> 21) & MASK_1BIT) as u8;
    out[22] = ((input >> 22) & MASK_1BIT) as u8;
    out[23] = ((input >> 23) & MASK_1BIT) as u8;
    out[24] = ((input >> 24) & MASK_1BIT) as u8;
    out[25] = ((input >> 25) & MASK_1BIT) as u8;
    out[26] = ((input >> 26) & MASK_1BIT) as u8;
    out[27] = ((input >> 27) & MASK_1BIT) as u8;
    out[28] = ((input >> 28) & MASK_1BIT) as u8;
    out[29] = ((input >> 29) & MASK_1BIT) as u8;
    out[30] = ((input >> 30) & MASK_1BIT) as u8;
    out[31] = ((input >> 31) & MASK_1BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 1-bit complex samples.
pub fn decode_1bit_complex(input: &u32) -> ([u8; 16], [u8; 16]) {
    let mut ip_out: [u8; 16] = [0; 16];
    let mut q_out: [u8; 16] = [0; 16];

    ip_out[0] = (input & MASK_1BIT) as u8;
    q_out[0] = ((input >> 1) & MASK_1BIT) as u8;
    ip_out[1] = ((input >> 2) & MASK_1BIT) as u8;
    q_out[1] = ((input >> 3) & MASK_1BIT) as u8;
    ip_out[2] = ((input >> 4) & MASK_1BIT) as u8;
    q_out[2] = ((input >> 5) & MASK_1BIT) as u8;
    ip_out[3] = ((input >> 6) & MASK_1BIT) as u8;
    q_out[3] = ((input >> 7) & MASK_1BIT) as u8;
    ip_out[4] = ((input >> 8) & MASK_1BIT) as u8;
    q_out[4] = ((input >> 9) & MASK_1BIT) as u8;
    ip_out[5] = ((input >> 10) & MASK_1BIT) as u8;
    q_out[5] = ((input >> 11) & MASK_1BIT) as u8;
    ip_out[6] = ((input >> 12) & MASK_1BIT) as u8;
    q_out[6] = ((input >> 13) & MASK_1BIT) as u8;
    ip_out[7] = ((input >> 14) & MASK_1BIT) as u8;
    q_out[7] = ((input >> 15) & MASK_1BIT) as u8;
    ip_out[8] = ((input >> 16) & MASK_1BIT) as u8;
    q_out[8] = ((input >> 17) & MASK_1BIT) as u8;
    ip_out[9] = ((input >> 18) & MASK_1BIT) as u8;
    q_out[9] = ((input >> 19) & MASK_1BIT) as u8;
    ip_out[10] = ((input >> 20) & MASK_1BIT) as u8;
    q_out[10] = ((input >> 21) & MASK_1BIT) as u8;
    ip_out[11] = ((input >> 22) & MASK_1BIT) as u8;
    q_out[11] = ((input >> 23) & MASK_1BIT) as u8;
    ip_out[12] = ((input >> 24) & MASK_1BIT) as u8;
    q_out[12] = ((input >> 25) & MASK_1BIT) as u8;
    ip_out[13] = ((input >> 26) & MASK_1BIT) as u8;
    q_out[13] = ((input >> 27) & MASK_1BIT) as u8;
    ip_out[14] = ((input >> 28) & MASK_1BIT) as u8;
    q_out[14] = ((input >> 29) & MASK_1BIT) as u8;
    ip_out[15] = ((input >> 30) & MASK_1BIT) as u8;
    q_out[15] = ((input >> 31) & MASK_1BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 2-bit real samples.
pub fn decode_2bit_real(input: &u32) -> [u8; 16] {
    let mut out: [u8; 16] = [0; 16];

    out[0] = (input & MASK_2BIT) as u8;
    out[1] = ((input >> 2) & MASK_2BIT) as u8;
    out[2] = ((input >> 4) & MASK_2BIT) as u8;
    out[3] = ((input >> 6) & MASK_2BIT) as u8;
    out[4] = ((input >> 8) & MASK_2BIT) as u8;
    out[5] = ((input >> 10) & MASK_2BIT) as u8;
    out[6] = ((input >> 12) & MASK_2BIT) as u8;
    out[7] = ((input >> 14) & MASK_2BIT) as u8;
    out[8] = ((input >> 16) & MASK_2BIT) as u8;
    out[9] = ((input >> 18) & MASK_2BIT) as u8;
    out[10] = ((input >> 20) & MASK_2BIT) as u8;
    out[11] = ((input >> 22) & MASK_2BIT) as u8;
    out[12] = ((input >> 24) & MASK_2BIT) as u8;
    out[13] = ((input >> 26) & MASK_2BIT) as u8;
    out[14] = ((input >> 28) & MASK_2BIT) as u8;
    out[15] = ((input >> 30) & MASK_2BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 2-bit complex samples.
pub fn decode_2bit_complex(input: &u32) -> ([u8; 8], [u8; 8]) {
    let mut ip_out: [u8; 8] = [0; 8];
    let mut q_out: [u8; 8] = [0; 8];

    ip_out[0] = (input & MASK_2BIT) as u8;
    q_out[0] = ((input >> 2) & MASK_2BIT) as u8;
    ip_out[1] = ((input >> 4) & MASK_2BIT) as u8;
    q_out[1] = ((input >> 6) & MASK_2BIT) as u8;
    ip_out[2] = ((input >> 8) & MASK_2BIT) as u8;
    q_out[2] = ((input >> 10) & MASK_2BIT) as u8;
    ip_out[3] = ((input >> 12) & MASK_2BIT) as u8;
    q_out[3] = ((input >> 14) & MASK_2BIT) as u8;
    ip_out[4] = ((input >> 16) & MASK_2BIT) as u8;
    q_out[4] = ((input >> 18) & MASK_2BIT) as u8;
    ip_out[5] = ((input >> 20) & MASK_2BIT) as u8;
    q_out[5] = ((input >> 22) & MASK_2BIT) as u8;
    ip_out[6] = ((input >> 24) & MASK_2BIT) as u8;
    q_out[6] = ((input >> 26) & MASK_2BIT) as u8;
    ip_out[7] = ((input >> 28) & MASK_2BIT) as u8;
    q_out[7] = ((input >> 30) & MASK_2BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 3-bit real samples.
pub fn decode_3bit_real(input: &u32) -> [u8; 10] {
    let mut out: [u8; 10] = [0; 10];

    out[0] = (input & MASK_3BIT) as u8;
    out[1] = ((input >> 3) & MASK_3BIT) as u8;
    out[2] = ((input >> 6) & MASK_3BIT) as u8;
    out[3] = ((input >> 9) & MASK_3BIT) as u8;
    out[4] = ((input >> 12) & MASK_3BIT) as u8;
    out[5] = ((input >> 15) & MASK_3BIT) as u8;
    out[6] = ((input >> 18) & MASK_3BIT) as u8;
    out[7] = ((input >> 21) & MASK_3BIT) as u8;
    out[8] = ((input >> 24) & MASK_3BIT) as u8;
    out[9] = ((input >> 27) & MASK_3BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 3-bit complex samples.
pub fn decode_3bit_complex(input: &u32) -> ([u8; 5], [u8; 5]) {
    let mut ip_out: [u8; 5] = [0; 5];
    let mut q_out: [u8; 5] = [0; 5];

    ip_out[0] = (input & MASK_3BIT) as u8;
    q_out[0] = ((input >> 3) & MASK_3BIT) as u8;
    ip_out[1] = ((input >> 6) & MASK_3BIT) as u8;
    q_out[1] = ((input >> 9) & MASK_3BIT) as u8;
    ip_out[2] = ((input >> 12) & MASK_3BIT) as u8;
    q_out[2] = ((input >> 15) & MASK_3BIT) as u8;
    ip_out[3] = ((input >> 18) & MASK_3BIT) as u8;
    q_out[3] = ((input >> 21) & MASK_3BIT) as u8;
    ip_out[4] = ((input >> 24) & MASK_3BIT) as u8;
    q_out[4] = ((input >> 27) & MASK_3BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 4-bit real samples.
pub fn decode_4bit_real(input: &u32) -> [u8; 8] {
    let mut out: [u8; 8] = [0; 8];

    out[0] = (input & MASK_4BIT) as u8;
    out[1] = ((input >> 4) & MASK_4BIT) as u8;
    out[2] = ((input >> 8) & MASK_4BIT) as u8;
    out[3] = ((input >> 12) & MASK_4BIT) as u8;
    out[4] = ((input >> 16) & MASK_4BIT) as u8;
    out[5] = ((input >> 20) & MASK_4BIT) as u8;
    out[6] = ((input >> 24) & MASK_4BIT) as u8;
    out[7] = ((input >> 28) & MASK_4BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 4-bit complex samples.
pub fn decode_4bit_complex(input: &u32) -> ([u8; 4], [u8; 4]) {
    let mut ip_out: [u8; 4] = [0; 4];
    let mut q_out: [u8; 4] = [0; 4];

    ip_out[0] = (input & MASK_4BIT) as u8;
    q_out[0] = ((input >> 4) & MASK_4BIT) as u8;
    ip_out[1] = ((input >> 8) & MASK_4BIT) as u8;
    q_out[1] = ((input >> 12) & MASK_4BIT) as u8;
    ip_out[2] = ((input >> 16) & MASK_4BIT) as u8;
    q_out[2] = ((input >> 20) & MASK_4BIT) as u8;
    ip_out[3] = ((input >> 24) & MASK_4BIT) as u8;
    q_out[3] = ((input >> 28) & MASK_4BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 6-bit real samples.
pub fn decode_6bit_real(input: &u32) -> [u8; 5] {
    let mut out: [u8; 5] = [0; 5];

    out[0] = (input & MASK_6BIT) as u8;
    out[1] = ((input >> 6) & MASK_6BIT) as u8;
    out[2] = ((input >> 12) & MASK_6BIT) as u8;
    out[3] = ((input >> 18) & MASK_6BIT) as u8;
    out[4] = ((input >> 24) & MASK_6BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 6-bit complex samples.
pub fn decode_6bit_complex(input: &u32) -> ([u8; 2], [u8; 2]) {
    let mut ip_out: [u8; 2] = [0; 2];
    let mut q_out: [u8; 2] = [0; 2];

    ip_out[0] = (input & MASK_6BIT) as u8;
    q_out[0] = ((input >> 6) & MASK_6BIT) as u8;
    ip_out[1] = ((input >> 12) & MASK_6BIT) as u8;
    q_out[1] = ((input >> 18) & MASK_6BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 7-bit real samples.
pub fn decode_7bit_real(input: &u32) -> [u8; 4] {
    let mut out: [u8; 4] = [0; 4];

    out[0] = (input & MASK_7BIT) as u8;
    out[1] = ((input >> 7) & MASK_7BIT) as u8;
    out[2] = ((input >> 14) & MASK_7BIT) as u8;
    out[3] = ((input >> 21) & MASK_7BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 7-bit complex samples.
pub fn decode_7bit_complex(input: &u32) -> ([u8; 2], [u8; 2]) {
    let mut ip_out: [u8; 2] = [0; 2];
    let mut q_out: [u8; 2] = [0; 2];

    ip_out[0] = (input & MASK_7BIT) as u8;
    q_out[0] = ((input >> 7) & MASK_7BIT) as u8;
    ip_out[1] = ((input >> 14) & MASK_7BIT) as u8;
    q_out[1] = ((input >> 21) & MASK_7BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 8-bit real samples.
pub fn decode_8bit_real(input: &u32) -> [u8; 4] {
    let mut out: [u8; 4] = [0; 4];

    out[0] = (input & MASK_8BIT) as u8;
    out[1] = ((input >> 8) & MASK_8BIT) as u8;
    out[2] = ((input >> 16) & MASK_8BIT) as u8;
    out[3] = ((input >> 24) & MASK_8BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 8-bit complex samples.
pub fn decode_8bit_complex(input: &u32) -> ([u8; 2], [u8; 2]) {
    let mut ip_out: [u8; 2] = [0; 2];
    let mut q_out: [u8; 2] = [0; 2];

    ip_out[0] = (input & MASK_8BIT) as u8;
    q_out[0] = ((input >> 8) & MASK_8BIT) as u8;
    ip_out[1] = ((input >> 16) & MASK_8BIT) as u8;
    q_out[1] = ((input >> 24) & MASK_8BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 11-bit real samples.
pub fn decode_11bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & MASK_11BIT) as u16;
    out[1] = ((input >> 11) & MASK_11BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 11-bit real samples.
pub fn decode_11bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & MASK_11BIT) as u16,
        ((input >> 11) & MASK_11BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 12-bit real samples.
pub fn decode_12bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & MASK_12BIT) as u16;
    out[1] = ((input >> 12) & MASK_12BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 12-bit real samples.
pub fn decode_12bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & MASK_12BIT) as u16,
        ((input >> 12) & MASK_12BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 13-bit real samples.
pub fn decode_13bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & MASK_13BIT) as u16;
    out[1] = ((input >> 13) & MASK_13BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 13-bit real samples.
pub fn decode_13bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & MASK_13BIT) as u16,
        ((input >> 13) & MASK_13BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 14-bit real samples.
pub fn decode_14bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & MASK_14BIT) as u16;
    out[1] = ((input >> 14) & MASK_14BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 14-bit real samples.
pub fn decode_14bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & MASK_14BIT) as u16,
        ((input >> 14) & MASK_14BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 15-bit real samples.
pub fn decode_15bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & MASK_15BIT) as u16;
    out[1] = ((input >> 15) & MASK_15BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 15-bit real samples.
pub fn decode_15bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & MASK_15BIT) as u16,
        ((input >> 15) & MASK_15BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 16-bit real samples.
pub fn decode_16bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & MASK_16BIT) as u16;
    out[1] = ((input >> 16) & MASK_16BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 16-bit real samples.
pub fn decode_16bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & MASK_16BIT) as u16,
        ((input >> 16) & MASK_16BIT) as u16,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_header() {
        let example_header = VDIFHeader {
            is_valid: true,
            is_legacy: false,
            time: 100,
            epoch: 10,
            frame: 500,
            version: 3,
            channels: 16,
            size: 8032,
            is_real: true,
            bits_per_sample: 4,
            thread: 64,
            station: 50764,
            edv0: 0,
            edv1: 0,
            edv2: 0,
            edv3: 0,
        };

        // Encode and then decode to make sure it's the same.
        let cpy = example_header.clone();
        let encoded = encode_header(cpy);
        let (_, parsed) = parse_header(&encoded).unwrap();

        assert_eq!(example_header, parsed)
    }

    #[test]
    fn test_decode_1bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 32] = [
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0,
        ];
        assert_eq!(decode_1bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_1bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 16], [u8; 16]) = (
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(decode_1bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_2bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(decode_2bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_2bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 8], [u8; 8]) = ([1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(decode_2bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_3bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 10] = [5, 2, 5, 2, 5, 2, 5, 2, 5, 2];
        assert_eq!(decode_3bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_3bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 5], [u8; 5]) = ([5, 5, 5, 5, 5], [2, 2, 2, 2, 2]);
        assert_eq!(decode_3bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_4bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 8] = [5, 5, 5, 5, 5, 5, 5, 5];
        assert_eq!(decode_4bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_4bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 4], [u8; 4]) = ([5, 5, 5, 5], [5, 5, 5, 5]);
        assert_eq!(decode_4bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_6bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 5] = [21, 21, 21, 21, 21];
        assert_eq!(decode_6bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_6bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 2], [u8; 2]) = ([21, 21], [21, 21]);
        assert_eq!(decode_6bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_7bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 4] = [85, 42, 85, 42];
        assert_eq!(decode_7bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_7bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 2], [u8; 2]) = ([85, 85], [42, 42]);
        assert_eq!(decode_7bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_8bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 4] = [85, 85, 85, 85];
        assert_eq!(decode_8bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_8bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 2], [u8; 2]) = ([85, 85], [85, 85]);
        assert_eq!(decode_8bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_11bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u16; 2] = [0b10101010101, 0b01010101010];
        assert_eq!(decode_11bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_11bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: (u16, u16) = (0b10101010101, 0b01010101010);
        assert_eq!(decode_11bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_12bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u16; 2] = [0b010101010101, 0b010101010101];
        assert_eq!(decode_12bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_12bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: (u16, u16) = (0b010101010101, 0b010101010101);
        assert_eq!(decode_12bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_13bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u16; 2] = [0b1010101010101, 0b0101010101010];
        assert_eq!(decode_13bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_13bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: (u16, u16) = (0b1010101010101, 0b0101010101010);
        assert_eq!(decode_13bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_14bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u16; 2] = [0b01010101010101, 0b01010101010101];
        assert_eq!(decode_14bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_14bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: (u16, u16) = (0b01010101010101, 0b01010101010101);
        assert_eq!(decode_14bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_15bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u16; 2] = [0b101010101010101, 0b010101010101010];
        assert_eq!(decode_15bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_15bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: (u16, u16) = (0b101010101010101, 0b010101010101010);
        assert_eq!(decode_15bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_16bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u16; 2] = [0b0101010101010101, 0b0101010101010101];
        assert_eq!(decode_16bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_16bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: (u16, u16) = (0b0101010101010101, 0b0101010101010101);
        assert_eq!(decode_16bit_complex(&test_in), result)
    }
}
