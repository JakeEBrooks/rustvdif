//! Provides functionality for encoding/decoding VDIF payloads.
//!
//! Note that these functions *may* not be the most performant way of doing what you need, but are provided for
//! convenience, or for when you just want to inspect a VDIF frame's payload.
//!
//! Up to 16-bit encoding is supported, but let me know on GitHub if you have a use case for larger bits/sample. 
//! While this crate supports uncommon bits per sample like 6 bit, you should try to stick to 2^n bits per sample
//! (i.e. 1, 2, 4, 8, 16, 32) since they are more efficient to store in VDIF.
//! 
//! Decoded samples are in chronological order, i.e. the most recent sample occupies the largest array index.

// Other VDIF software uses a LUT for decoding the u32 word, but
// writing it out as below seems to be at least the same speed, if not faster.
// This is tested for 1 bit decoding, I assume it holds for higher bit depths since
// they require less operations than 1 bit.

// The decoding functions are quite ugly, but writing it out explicitly
// maximises information to the compiler and doesn't run the risk of
// loops not being unrolled

// The VDIF spec doesn't seem to be clear on how the encoding is applied in cases
// where the number of real samples is not exactly 2x the number of complex samples.
// For example, in 6-bit encoding, you can fit 5 real samples, but only 2 complex samples
// (otherwise a real component would not have an attached complex component).
// In these cases I take the safer approach and maintain the extra real sample.

const DC_MASK_1BIT: u32 = u32::MAX >> 31;
const DC_MASK_2BIT: u32 = u32::MAX >> 30;
const DC_MASK_3BIT: u32 = u32::MAX >> 29;
const DC_MASK_4BIT: u32 = u32::MAX >> 28;

const DC_MASK_6BIT: u32 = u32::MAX >> 26;
const DC_MASK_7BIT: u32 = u32::MAX >> 25;
const DC_MASK_8BIT: u32 = u32::MAX >> 24;

const DC_MASK_11BIT: u32 = u32::MAX >> 21;
const DC_MASK_12BIT: u32 = u32::MAX >> 20;
const DC_MASK_13BIT: u32 = u32::MAX >> 19;
const DC_MASK_14BIT: u32 = u32::MAX >> 18;
const DC_MASK_15BIT: u32 = u32::MAX >> 17;
const DC_MASK_16BIT: u32 = u32::MAX >> 16;

const EC_MASK_1BIT: u8 = 1;
const EC_MASK_2BIT: u8 = 2u8.pow(2) - 1;
const EC_MASK_3BIT: u8 = 2u8.pow(3) - 1;
const EC_MASK_4BIT: u8 = 2u8.pow(4) - 1;

const EC_MASK_6BIT: u8 = 2u8.pow(6) - 1;
const EC_MASK_7BIT: u8 = 2u8.pow(7) - 1;

const EC_MASK_11BIT: u16 = 2u16.pow(11) - 1;
const EC_MASK_12BIT: u16 = 2u16.pow(12) - 1;
const EC_MASK_13BIT: u16 = 2u16.pow(13) - 1;
const EC_MASK_14BIT: u16 = 2u16.pow(14) - 1;
const EC_MASK_15BIT: u16 = 2u16.pow(15) - 1;

/// Decode a VDIF encoded 32-bit word of 1-bit real samples.
pub fn decode_1bit_real(input: &u32) -> [u8; 32] {
    let mut out: [u8; 32] = [0; 32];

    out[0] = (input & DC_MASK_1BIT) as u8;
    out[1] = ((input >> 1) & DC_MASK_1BIT) as u8;
    out[2] = ((input >> 2) & DC_MASK_1BIT) as u8;
    out[3] = ((input >> 3) & DC_MASK_1BIT) as u8;
    out[4] = ((input >> 4) & DC_MASK_1BIT) as u8;
    out[5] = ((input >> 5) & DC_MASK_1BIT) as u8;
    out[6] = ((input >> 6) & DC_MASK_1BIT) as u8;
    out[7] = ((input >> 7) & DC_MASK_1BIT) as u8;
    out[8] = ((input >> 8) & DC_MASK_1BIT) as u8;
    out[9] = ((input >> 9) & DC_MASK_1BIT) as u8;
    out[10] = ((input >> 10) & DC_MASK_1BIT) as u8;
    out[11] = ((input >> 11) & DC_MASK_1BIT) as u8;
    out[12] = ((input >> 12) & DC_MASK_1BIT) as u8;
    out[13] = ((input >> 13) & DC_MASK_1BIT) as u8;
    out[14] = ((input >> 14) & DC_MASK_1BIT) as u8;
    out[15] = ((input >> 15) & DC_MASK_1BIT) as u8;
    out[16] = ((input >> 16) & DC_MASK_1BIT) as u8;
    out[17] = ((input >> 17) & DC_MASK_1BIT) as u8;
    out[18] = ((input >> 18) & DC_MASK_1BIT) as u8;
    out[19] = ((input >> 19) & DC_MASK_1BIT) as u8;
    out[20] = ((input >> 20) & DC_MASK_1BIT) as u8;
    out[21] = ((input >> 21) & DC_MASK_1BIT) as u8;
    out[22] = ((input >> 22) & DC_MASK_1BIT) as u8;
    out[23] = ((input >> 23) & DC_MASK_1BIT) as u8;
    out[24] = ((input >> 24) & DC_MASK_1BIT) as u8;
    out[25] = ((input >> 25) & DC_MASK_1BIT) as u8;
    out[26] = ((input >> 26) & DC_MASK_1BIT) as u8;
    out[27] = ((input >> 27) & DC_MASK_1BIT) as u8;
    out[28] = ((input >> 28) & DC_MASK_1BIT) as u8;
    out[29] = ((input >> 29) & DC_MASK_1BIT) as u8;
    out[30] = ((input >> 30) & DC_MASK_1BIT) as u8;
    out[31] = ((input >> 31) & DC_MASK_1BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 1-bit complex samples.
pub fn decode_1bit_complex(input: &u32) -> ([u8; 16], [u8; 16]) {
    let mut ip_out: [u8; 16] = [0; 16];
    let mut q_out: [u8; 16] = [0; 16];

    ip_out[0] = (input & DC_MASK_1BIT) as u8;
    q_out[0] = ((input >> 1) & DC_MASK_1BIT) as u8;
    ip_out[1] = ((input >> 2) & DC_MASK_1BIT) as u8;
    q_out[1] = ((input >> 3) & DC_MASK_1BIT) as u8;
    ip_out[2] = ((input >> 4) & DC_MASK_1BIT) as u8;
    q_out[2] = ((input >> 5) & DC_MASK_1BIT) as u8;
    ip_out[3] = ((input >> 6) & DC_MASK_1BIT) as u8;
    q_out[3] = ((input >> 7) & DC_MASK_1BIT) as u8;
    ip_out[4] = ((input >> 8) & DC_MASK_1BIT) as u8;
    q_out[4] = ((input >> 9) & DC_MASK_1BIT) as u8;
    ip_out[5] = ((input >> 10) & DC_MASK_1BIT) as u8;
    q_out[5] = ((input >> 11) & DC_MASK_1BIT) as u8;
    ip_out[6] = ((input >> 12) & DC_MASK_1BIT) as u8;
    q_out[6] = ((input >> 13) & DC_MASK_1BIT) as u8;
    ip_out[7] = ((input >> 14) & DC_MASK_1BIT) as u8;
    q_out[7] = ((input >> 15) & DC_MASK_1BIT) as u8;
    ip_out[8] = ((input >> 16) & DC_MASK_1BIT) as u8;
    q_out[8] = ((input >> 17) & DC_MASK_1BIT) as u8;
    ip_out[9] = ((input >> 18) & DC_MASK_1BIT) as u8;
    q_out[9] = ((input >> 19) & DC_MASK_1BIT) as u8;
    ip_out[10] = ((input >> 20) & DC_MASK_1BIT) as u8;
    q_out[10] = ((input >> 21) & DC_MASK_1BIT) as u8;
    ip_out[11] = ((input >> 22) & DC_MASK_1BIT) as u8;
    q_out[11] = ((input >> 23) & DC_MASK_1BIT) as u8;
    ip_out[12] = ((input >> 24) & DC_MASK_1BIT) as u8;
    q_out[12] = ((input >> 25) & DC_MASK_1BIT) as u8;
    ip_out[13] = ((input >> 26) & DC_MASK_1BIT) as u8;
    q_out[13] = ((input >> 27) & DC_MASK_1BIT) as u8;
    ip_out[14] = ((input >> 28) & DC_MASK_1BIT) as u8;
    q_out[14] = ((input >> 29) & DC_MASK_1BIT) as u8;
    ip_out[15] = ((input >> 30) & DC_MASK_1BIT) as u8;
    q_out[15] = ((input >> 31) & DC_MASK_1BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 2-bit real samples.
pub fn decode_2bit_real(input: &u32) -> [u8; 16] {
    let mut out: [u8; 16] = [0; 16];

    out[0] = (input & DC_MASK_2BIT) as u8;
    out[1] = ((input >> 2) & DC_MASK_2BIT) as u8;
    out[2] = ((input >> 4) & DC_MASK_2BIT) as u8;
    out[3] = ((input >> 6) & DC_MASK_2BIT) as u8;
    out[4] = ((input >> 8) & DC_MASK_2BIT) as u8;
    out[5] = ((input >> 10) & DC_MASK_2BIT) as u8;
    out[6] = ((input >> 12) & DC_MASK_2BIT) as u8;
    out[7] = ((input >> 14) & DC_MASK_2BIT) as u8;
    out[8] = ((input >> 16) & DC_MASK_2BIT) as u8;
    out[9] = ((input >> 18) & DC_MASK_2BIT) as u8;
    out[10] = ((input >> 20) & DC_MASK_2BIT) as u8;
    out[11] = ((input >> 22) & DC_MASK_2BIT) as u8;
    out[12] = ((input >> 24) & DC_MASK_2BIT) as u8;
    out[13] = ((input >> 26) & DC_MASK_2BIT) as u8;
    out[14] = ((input >> 28) & DC_MASK_2BIT) as u8;
    out[15] = ((input >> 30) & DC_MASK_2BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 2-bit complex samples.
pub fn decode_2bit_complex(input: &u32) -> ([u8; 8], [u8; 8]) {
    let mut ip_out: [u8; 8] = [0; 8];
    let mut q_out: [u8; 8] = [0; 8];

    ip_out[0] = (input & DC_MASK_2BIT) as u8;
    q_out[0] = ((input >> 2) & DC_MASK_2BIT) as u8;
    ip_out[1] = ((input >> 4) & DC_MASK_2BIT) as u8;
    q_out[1] = ((input >> 6) & DC_MASK_2BIT) as u8;
    ip_out[2] = ((input >> 8) & DC_MASK_2BIT) as u8;
    q_out[2] = ((input >> 10) & DC_MASK_2BIT) as u8;
    ip_out[3] = ((input >> 12) & DC_MASK_2BIT) as u8;
    q_out[3] = ((input >> 14) & DC_MASK_2BIT) as u8;
    ip_out[4] = ((input >> 16) & DC_MASK_2BIT) as u8;
    q_out[4] = ((input >> 18) & DC_MASK_2BIT) as u8;
    ip_out[5] = ((input >> 20) & DC_MASK_2BIT) as u8;
    q_out[5] = ((input >> 22) & DC_MASK_2BIT) as u8;
    ip_out[6] = ((input >> 24) & DC_MASK_2BIT) as u8;
    q_out[6] = ((input >> 26) & DC_MASK_2BIT) as u8;
    ip_out[7] = ((input >> 28) & DC_MASK_2BIT) as u8;
    q_out[7] = ((input >> 30) & DC_MASK_2BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 3-bit real samples.
pub fn decode_3bit_real(input: &u32) -> [u8; 10] {
    let mut out: [u8; 10] = [0; 10];

    out[0] = (input & DC_MASK_3BIT) as u8;
    out[1] = ((input >> 3) & DC_MASK_3BIT) as u8;
    out[2] = ((input >> 6) & DC_MASK_3BIT) as u8;
    out[3] = ((input >> 9) & DC_MASK_3BIT) as u8;
    out[4] = ((input >> 12) & DC_MASK_3BIT) as u8;
    out[5] = ((input >> 15) & DC_MASK_3BIT) as u8;
    out[6] = ((input >> 18) & DC_MASK_3BIT) as u8;
    out[7] = ((input >> 21) & DC_MASK_3BIT) as u8;
    out[8] = ((input >> 24) & DC_MASK_3BIT) as u8;
    out[9] = ((input >> 27) & DC_MASK_3BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 3-bit complex samples.
pub fn decode_3bit_complex(input: &u32) -> ([u8; 5], [u8; 5]) {
    let mut ip_out: [u8; 5] = [0; 5];
    let mut q_out: [u8; 5] = [0; 5];

    ip_out[0] = (input & DC_MASK_3BIT) as u8;
    q_out[0] = ((input >> 3) & DC_MASK_3BIT) as u8;
    ip_out[1] = ((input >> 6) & DC_MASK_3BIT) as u8;
    q_out[1] = ((input >> 9) & DC_MASK_3BIT) as u8;
    ip_out[2] = ((input >> 12) & DC_MASK_3BIT) as u8;
    q_out[2] = ((input >> 15) & DC_MASK_3BIT) as u8;
    ip_out[3] = ((input >> 18) & DC_MASK_3BIT) as u8;
    q_out[3] = ((input >> 21) & DC_MASK_3BIT) as u8;
    ip_out[4] = ((input >> 24) & DC_MASK_3BIT) as u8;
    q_out[4] = ((input >> 27) & DC_MASK_3BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 4-bit real samples.
pub fn decode_4bit_real(input: &u32) -> [u8; 8] {
    let mut out: [u8; 8] = [0; 8];

    out[0] = (input & DC_MASK_4BIT) as u8;
    out[1] = ((input >> 4) & DC_MASK_4BIT) as u8;
    out[2] = ((input >> 8) & DC_MASK_4BIT) as u8;
    out[3] = ((input >> 12) & DC_MASK_4BIT) as u8;
    out[4] = ((input >> 16) & DC_MASK_4BIT) as u8;
    out[5] = ((input >> 20) & DC_MASK_4BIT) as u8;
    out[6] = ((input >> 24) & DC_MASK_4BIT) as u8;
    out[7] = ((input >> 28) & DC_MASK_4BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 4-bit complex samples.
pub fn decode_4bit_complex(input: &u32) -> ([u8; 4], [u8; 4]) {
    let mut ip_out: [u8; 4] = [0; 4];
    let mut q_out: [u8; 4] = [0; 4];

    ip_out[0] = (input & DC_MASK_4BIT) as u8;
    q_out[0] = ((input >> 4) & DC_MASK_4BIT) as u8;
    ip_out[1] = ((input >> 8) & DC_MASK_4BIT) as u8;
    q_out[1] = ((input >> 12) & DC_MASK_4BIT) as u8;
    ip_out[2] = ((input >> 16) & DC_MASK_4BIT) as u8;
    q_out[2] = ((input >> 20) & DC_MASK_4BIT) as u8;
    ip_out[3] = ((input >> 24) & DC_MASK_4BIT) as u8;
    q_out[3] = ((input >> 28) & DC_MASK_4BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 6-bit real samples.
pub fn decode_6bit_real(input: &u32) -> [u8; 5] {
    let mut out: [u8; 5] = [0; 5];

    out[0] = (input & DC_MASK_6BIT) as u8;
    out[1] = ((input >> 6) & DC_MASK_6BIT) as u8;
    out[2] = ((input >> 12) & DC_MASK_6BIT) as u8;
    out[3] = ((input >> 18) & DC_MASK_6BIT) as u8;
    out[4] = ((input >> 24) & DC_MASK_6BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 6-bit complex samples.
pub fn decode_6bit_complex(input: &u32) -> ([u8; 2], [u8; 2]) {
    let mut ip_out: [u8; 2] = [0; 2];
    let mut q_out: [u8; 2] = [0; 2];

    ip_out[0] = (input & DC_MASK_6BIT) as u8;
    q_out[0] = ((input >> 6) & DC_MASK_6BIT) as u8;
    ip_out[1] = ((input >> 12) & DC_MASK_6BIT) as u8;
    q_out[1] = ((input >> 18) & DC_MASK_6BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 7-bit real samples.
pub fn decode_7bit_real(input: &u32) -> [u8; 4] {
    let mut out: [u8; 4] = [0; 4];

    out[0] = (input & DC_MASK_7BIT) as u8;
    out[1] = ((input >> 7) & DC_MASK_7BIT) as u8;
    out[2] = ((input >> 14) & DC_MASK_7BIT) as u8;
    out[3] = ((input >> 21) & DC_MASK_7BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 7-bit complex samples.
pub fn decode_7bit_complex(input: &u32) -> ([u8; 2], [u8; 2]) {
    let mut ip_out: [u8; 2] = [0; 2];
    let mut q_out: [u8; 2] = [0; 2];

    ip_out[0] = (input & DC_MASK_7BIT) as u8;
    q_out[0] = ((input >> 7) & DC_MASK_7BIT) as u8;
    ip_out[1] = ((input >> 14) & DC_MASK_7BIT) as u8;
    q_out[1] = ((input >> 21) & DC_MASK_7BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 8-bit real samples.
pub fn decode_8bit_real(input: &u32) -> [u8; 4] {
    let mut out: [u8; 4] = [0; 4];

    out[0] = (input & DC_MASK_8BIT) as u8;
    out[1] = ((input >> 8) & DC_MASK_8BIT) as u8;
    out[2] = ((input >> 16) & DC_MASK_8BIT) as u8;
    out[3] = ((input >> 24) & DC_MASK_8BIT) as u8;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 8-bit complex samples.
pub fn decode_8bit_complex(input: &u32) -> ([u8; 2], [u8; 2]) {
    let mut ip_out: [u8; 2] = [0; 2];
    let mut q_out: [u8; 2] = [0; 2];

    ip_out[0] = (input & DC_MASK_8BIT) as u8;
    q_out[0] = ((input >> 8) & DC_MASK_8BIT) as u8;
    ip_out[1] = ((input >> 16) & DC_MASK_8BIT) as u8;
    q_out[1] = ((input >> 24) & DC_MASK_8BIT) as u8;

    return (ip_out, q_out);
}

/// Decode a VDIF encoded 32-bit word of 11-bit real samples.
pub fn decode_11bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & DC_MASK_11BIT) as u16;
    out[1] = ((input >> 11) & DC_MASK_11BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 11-bit real samples.
pub fn decode_11bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & DC_MASK_11BIT) as u16,
        ((input >> 11) & DC_MASK_11BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 12-bit real samples.
pub fn decode_12bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & DC_MASK_12BIT) as u16;
    out[1] = ((input >> 12) & DC_MASK_12BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 12-bit real samples.
pub fn decode_12bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & DC_MASK_12BIT) as u16,
        ((input >> 12) & DC_MASK_12BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 13-bit real samples.
pub fn decode_13bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & DC_MASK_13BIT) as u16;
    out[1] = ((input >> 13) & DC_MASK_13BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 13-bit real samples.
pub fn decode_13bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & DC_MASK_13BIT) as u16,
        ((input >> 13) & DC_MASK_13BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 14-bit real samples.
pub fn decode_14bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & DC_MASK_14BIT) as u16;
    out[1] = ((input >> 14) & DC_MASK_14BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 14-bit real samples.
pub fn decode_14bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & DC_MASK_14BIT) as u16,
        ((input >> 14) & DC_MASK_14BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 15-bit real samples.
pub fn decode_15bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & DC_MASK_15BIT) as u16;
    out[1] = ((input >> 15) & DC_MASK_15BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 15-bit real samples.
pub fn decode_15bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & DC_MASK_15BIT) as u16,
        ((input >> 15) & DC_MASK_15BIT) as u16,
    );
}

/// Decode a VDIF encoded 32-bit word of 16-bit real samples.
pub fn decode_16bit_real(input: &u32) -> [u16; 2] {
    let mut out: [u16; 2] = [0; 2];

    out[0] = (input & DC_MASK_16BIT) as u16;
    out[1] = ((input >> 16) & DC_MASK_16BIT) as u16;

    return out;
}

/// Decode a VDIF encoded 32-bit word of 16-bit real samples.
pub fn decode_16bit_complex(input: &u32) -> (u16, u16) {
    return (
        (input & DC_MASK_16BIT) as u16,
        ((input >> 16) & DC_MASK_16BIT) as u16,
    );
}

/// Encode 32 1-bit real samples into an array of bytes.
pub fn encode_1bit_real(input: [u8; 32]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_1BIT) as u32;
    word |= ((input[1] & EC_MASK_1BIT) as u32) << 1;
    word |= ((input[2] & EC_MASK_1BIT) as u32) << 2;
    word |= ((input[3] & EC_MASK_1BIT) as u32) << 3;
    word |= ((input[4] & EC_MASK_1BIT) as u32) << 4;
    word |= ((input[5] & EC_MASK_1BIT) as u32) << 5;
    word |= ((input[6] & EC_MASK_1BIT) as u32) << 6;
    word |= ((input[7] & EC_MASK_1BIT) as u32) << 7;
    word |= ((input[8] & EC_MASK_1BIT) as u32) << 8;
    word |= ((input[9] & EC_MASK_1BIT) as u32) << 9;
    word |= ((input[10] & EC_MASK_1BIT) as u32) << 10;
    word |= ((input[11] & EC_MASK_1BIT) as u32) << 11;
    word |= ((input[12] & EC_MASK_1BIT) as u32) << 12;
    word |= ((input[13] & EC_MASK_1BIT) as u32) << 13;
    word |= ((input[14] & EC_MASK_1BIT) as u32) << 14;
    word |= ((input[15] & EC_MASK_1BIT) as u32) << 15;
    word |= ((input[16] & EC_MASK_1BIT) as u32) << 16;
    word |= ((input[17] & EC_MASK_1BIT) as u32) << 17;
    word |= ((input[18] & EC_MASK_1BIT) as u32) << 18;
    word |= ((input[19] & EC_MASK_1BIT) as u32) << 19;
    word |= ((input[20] & EC_MASK_1BIT) as u32) << 20;
    word |= ((input[21] & EC_MASK_1BIT) as u32) << 21;
    word |= ((input[22] & EC_MASK_1BIT) as u32) << 22;
    word |= ((input[23] & EC_MASK_1BIT) as u32) << 23;
    word |= ((input[24] & EC_MASK_1BIT) as u32) << 24;
    word |= ((input[25] & EC_MASK_1BIT) as u32) << 25;
    word |= ((input[26] & EC_MASK_1BIT) as u32) << 26;
    word |= ((input[27] & EC_MASK_1BIT) as u32) << 27;
    word |= ((input[28] & EC_MASK_1BIT) as u32) << 28;
    word |= ((input[29] & EC_MASK_1BIT) as u32) << 29;
    word |= ((input[30] & EC_MASK_1BIT) as u32) << 30;
    word |= ((input[31] & EC_MASK_1BIT) as u32) << 31;

    return word.to_le_bytes()
}

/// Encode 16 1-bit complex samples into an array of bytes.
pub fn encode_1bit_complex(real: [u8; 16], imag: [u8; 16]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real[0] & EC_MASK_1BIT) as u32;
    word |= ((imag[0] & EC_MASK_1BIT) as u32) << 1;
    word |= ((real[1] & EC_MASK_1BIT) as u32) << 2;
    word |= ((imag[1] & EC_MASK_1BIT) as u32) << 3;
    word |= ((real[2] & EC_MASK_1BIT) as u32) << 4;
    word |= ((imag[2] & EC_MASK_1BIT) as u32) << 5;
    word |= ((real[3] & EC_MASK_1BIT) as u32) << 6;
    word |= ((imag[3] & EC_MASK_1BIT) as u32) << 7;
    word |= ((real[4] & EC_MASK_1BIT) as u32) << 8;
    word |= ((imag[4] & EC_MASK_1BIT) as u32) << 9;
    word |= ((real[5] & EC_MASK_1BIT) as u32) << 10;
    word |= ((imag[5] & EC_MASK_1BIT) as u32) << 11;
    word |= ((real[6] & EC_MASK_1BIT) as u32) << 12;
    word |= ((imag[6] & EC_MASK_1BIT) as u32) << 13;
    word |= ((real[7] & EC_MASK_1BIT) as u32) << 14;
    word |= ((imag[7] & EC_MASK_1BIT) as u32) << 15;
    word |= ((real[8] & EC_MASK_1BIT) as u32) << 16;
    word |= ((imag[8] & EC_MASK_1BIT) as u32) << 17;
    word |= ((real[9] & EC_MASK_1BIT) as u32) << 18;
    word |= ((imag[9] & EC_MASK_1BIT) as u32) << 19;
    word |= ((real[10] & EC_MASK_1BIT) as u32) << 20;
    word |= ((imag[10] & EC_MASK_1BIT) as u32) << 21;
    word |= ((real[11] & EC_MASK_1BIT) as u32) << 22;
    word |= ((imag[11] & EC_MASK_1BIT) as u32) << 23;
    word |= ((real[12] & EC_MASK_1BIT) as u32) << 24;
    word |= ((imag[12] & EC_MASK_1BIT) as u32) << 25;
    word |= ((real[13] & EC_MASK_1BIT) as u32) << 26;
    word |= ((imag[13] & EC_MASK_1BIT) as u32) << 27;
    word |= ((real[14] & EC_MASK_1BIT) as u32) << 28;
    word |= ((imag[14] & EC_MASK_1BIT) as u32) << 29;
    word |= ((real[15] & EC_MASK_1BIT) as u32) << 30;
    word |= ((imag[15] & EC_MASK_1BIT) as u32) << 31;

    return word.to_le_bytes()
}

/// Encode 16 2-bit real samples into an array of bytes.
pub fn encode_2bit_real(input: [u8; 16]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_2BIT) as u32;
    word |= ((input[1] & EC_MASK_2BIT) as u32) << 2;
    word |= ((input[2] & EC_MASK_2BIT) as u32) << 4;
    word |= ((input[3] & EC_MASK_2BIT) as u32) << 6;
    word |= ((input[4] & EC_MASK_2BIT) as u32) << 8;
    word |= ((input[5] & EC_MASK_2BIT) as u32) << 10;
    word |= ((input[6] & EC_MASK_2BIT) as u32) << 12;
    word |= ((input[7] & EC_MASK_2BIT) as u32) << 14;
    word |= ((input[8] & EC_MASK_2BIT) as u32) << 16;
    word |= ((input[9] & EC_MASK_2BIT) as u32) << 18;
    word |= ((input[10] & EC_MASK_2BIT) as u32) << 20;
    word |= ((input[11] & EC_MASK_2BIT) as u32) << 22;
    word |= ((input[12] & EC_MASK_2BIT) as u32) << 24;
    word |= ((input[13] & EC_MASK_2BIT) as u32) << 26;
    word |= ((input[14] & EC_MASK_2BIT) as u32) << 28;
    word |= ((input[15] & EC_MASK_2BIT) as u32) << 30;

    return word.to_le_bytes()
}

/// Encode 8 2-bit complex samples into an array of bytes.
pub fn encode_2bit_complex(real: [u8; 8], imag: [u8; 8]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real[0] & EC_MASK_2BIT) as u32;
    word |= ((imag[0] & EC_MASK_2BIT) as u32) << 2;
    word |= ((real[1] & EC_MASK_2BIT) as u32) << 4;
    word |= ((imag[1] & EC_MASK_2BIT) as u32) << 6;
    word |= ((real[2] & EC_MASK_2BIT) as u32) << 8;
    word |= ((imag[2] & EC_MASK_2BIT) as u32) << 10;
    word |= ((real[3] & EC_MASK_2BIT) as u32) << 12;
    word |= ((imag[3] & EC_MASK_2BIT) as u32) << 14;
    word |= ((real[4] & EC_MASK_2BIT) as u32) << 16;
    word |= ((imag[4] & EC_MASK_2BIT) as u32) << 18;
    word |= ((real[5] & EC_MASK_2BIT) as u32) << 20;
    word |= ((imag[5] & EC_MASK_2BIT) as u32) << 22;
    word |= ((real[6] & EC_MASK_2BIT) as u32) << 24;
    word |= ((imag[6] & EC_MASK_2BIT) as u32) << 26;
    word |= ((real[7] & EC_MASK_2BIT) as u32) << 28;
    word |= ((imag[7] & EC_MASK_2BIT) as u32) << 30;

    return word.to_le_bytes()
}

/// Encode 10 3-bit real samples into an array of bytes.
pub fn encode_3bit_real(input: [u8; 10]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_3BIT) as u32;
    word |= ((input[1] & EC_MASK_3BIT) as u32) << 3;
    word |= ((input[2] & EC_MASK_3BIT) as u32) << 6;
    word |= ((input[3] & EC_MASK_3BIT) as u32) << 9;
    word |= ((input[4] & EC_MASK_3BIT) as u32) << 12;
    word |= ((input[5] & EC_MASK_3BIT) as u32) << 15;
    word |= ((input[6] & EC_MASK_3BIT) as u32) << 18;
    word |= ((input[7] & EC_MASK_3BIT) as u32) << 21;
    word |= ((input[8] & EC_MASK_3BIT) as u32) << 24;
    word |= ((input[9] & EC_MASK_3BIT) as u32) << 27;

    return word.to_le_bytes()
}

/// Encode 5 3-bit complex samples into an array of bytes.
pub fn encode_3bit_complex(real: [u8; 5], imag: [u8; 5]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real[0] & EC_MASK_3BIT) as u32;
    word |= ((imag[0] & EC_MASK_3BIT) as u32) << 3;
    word |= ((real[1] & EC_MASK_3BIT) as u32) << 6;
    word |= ((imag[1] & EC_MASK_3BIT) as u32) << 9;
    word |= ((real[2] & EC_MASK_3BIT) as u32) << 12;
    word |= ((imag[2] & EC_MASK_3BIT) as u32) << 15;
    word |= ((real[3] & EC_MASK_3BIT) as u32) << 18;
    word |= ((imag[3] & EC_MASK_3BIT) as u32) << 21;
    word |= ((real[4] & EC_MASK_3BIT) as u32) << 24;
    word |= ((imag[4] & EC_MASK_3BIT) as u32) << 27;

    return word.to_le_bytes()
}

/// Encode 8 4-bit real samples into an array of bytes.
pub fn encode_4bit_real(input: [u8; 8]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_4BIT) as u32;
    word |= ((input[1] & EC_MASK_4BIT) as u32) << 4;
    word |= ((input[2] & EC_MASK_4BIT) as u32) << 8;
    word |= ((input[3] & EC_MASK_4BIT) as u32) << 12;
    word |= ((input[4] & EC_MASK_4BIT) as u32) << 16;
    word |= ((input[5] & EC_MASK_4BIT) as u32) << 20;
    word |= ((input[6] & EC_MASK_4BIT) as u32) << 24;
    word |= ((input[7] & EC_MASK_4BIT) as u32) << 28;

    return word.to_le_bytes()
}

/// Encode 4 4-bit complex samples into an array of bytes.
pub fn encode_4bit_complex(real: [u8; 4], imag: [u8; 4]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real[0] & EC_MASK_4BIT) as u32;
    word |= ((imag[0] & EC_MASK_4BIT) as u32) << 4;
    word |= ((real[1] & EC_MASK_4BIT) as u32) << 8;
    word |= ((imag[1] & EC_MASK_4BIT) as u32) << 12;
    word |= ((real[2] & EC_MASK_4BIT) as u32) << 16;
    word |= ((imag[2] & EC_MASK_4BIT) as u32) << 20;
    word |= ((real[3] & EC_MASK_4BIT) as u32) << 24;
    word |= ((imag[3] & EC_MASK_4BIT) as u32) << 28;

    return word.to_le_bytes()
}

/// Encode 5 6-bit real samples into an array of bytes.
pub fn encode_6bit_real(input: [u8; 5]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_6BIT) as u32;
    word |= ((input[1] & EC_MASK_6BIT) as u32) << 6;
    word |= ((input[2] & EC_MASK_6BIT) as u32) << 12;
    word |= ((input[3] & EC_MASK_6BIT) as u32) << 18;
    word |= ((input[4] & EC_MASK_6BIT) as u32) << 24;

    return word.to_le_bytes()
}

/// Encode 2 6-bit complex samples into an array of bytes.
pub fn encode_6bit_complex(real: [u8; 2], imag: [u8; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real[0] & EC_MASK_6BIT) as u32;
    word |= ((imag[0] & EC_MASK_6BIT) as u32) << 6;
    word |= ((real[1] & EC_MASK_6BIT) as u32) << 12;
    word |= ((imag[1] & EC_MASK_6BIT) as u32) << 18;

    return word.to_le_bytes()
}

/// Encode 4 7-bit real samples into an array of bytes.
pub fn encode_7bit_real(input: [u8; 4]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_7BIT) as u32;
    word |= ((input[1] & EC_MASK_7BIT) as u32) << 7;
    word |= ((input[2] & EC_MASK_7BIT) as u32) << 14;
    word |= ((input[3] & EC_MASK_7BIT) as u32) << 21;

    return word.to_le_bytes()
}

/// Encode 2 7-bit complex samples into an array of bytes.
pub fn encode_7bit_complex(real: [u8; 2], imag: [u8; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real[0] & EC_MASK_7BIT) as u32;
    word |= ((imag[0] & EC_MASK_7BIT) as u32) << 7;
    word |= ((real[1] & EC_MASK_7BIT) as u32) << 14;
    word |= ((imag[1] & EC_MASK_7BIT) as u32) << 21;

    return word.to_le_bytes()
}

/// Encode 4 8-bit real samples into an array of bytes.
pub fn encode_8bit_real(input: [u8; 4]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= input[0] as u32;
    word |= (input[1] as u32) << 8;
    word |= (input[2] as u32) << 16;
    word |= (input[3] as u32) << 24;

    return word.to_le_bytes()
}

/// Encode 2 8-bit complex samples into an array of bytes.
pub fn encode_8bit_complex(real: [u8; 2], imag: [u8; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= real[0] as u32;
    word |= (imag[0] as u32) << 8;
    word |= (real[1] as u32) << 16;
    word |= (imag[1] as u32) << 24;

    return word.to_le_bytes()
}

/// Encode 2 11-bit real samples into an array of bytes.
pub fn encode_11bit_real(input: [u16; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_11BIT) as u32;
    word |= ((input[1] & EC_MASK_11BIT) as u32) << 11;

    return word.to_le_bytes()
}

/// Encode an 11-bit complex sample into an array of bytes.
pub fn encode_11bit_complex(real: u16, imag: u16) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real & EC_MASK_11BIT) as u32;
    word |= ((imag & EC_MASK_11BIT) as u32) << 11;

    return word.to_le_bytes()
}

/// Encode 2 12-bit real samples into an array of bytes.
pub fn encode_12bit_real(input: [u16; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_12BIT) as u32;
    word |= ((input[1] & EC_MASK_12BIT) as u32) << 12;

    return word.to_le_bytes()
}

/// Encode a 12-bit complex sample into an array of bytes.
pub fn encode_12bit_complex(real: u16, imag: u16) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real & EC_MASK_12BIT) as u32;
    word |= ((imag & EC_MASK_12BIT) as u32) << 12;

    return word.to_le_bytes()
}

/// Encode 2 13-bit real samples into an array of bytes.
pub fn encode_13bit_real(input: [u16; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_13BIT) as u32;
    word |= ((input[1] & EC_MASK_13BIT) as u32) << 13;

    return word.to_le_bytes()
}

/// Encode a 13-bit complex sample into an array of bytes.
pub fn encode_13bit_complex(real: u16, imag: u16) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real & EC_MASK_13BIT) as u32;
    word |= ((imag & EC_MASK_13BIT) as u32) << 13;

    return word.to_le_bytes()
}

/// Encode 2 14-bit real samples into an array of bytes.
pub fn encode_14bit_real(input: [u16; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_14BIT) as u32;
    word |= ((input[1] & EC_MASK_14BIT) as u32) << 14;

    return word.to_le_bytes()
}

/// Encode a 14-bit complex sample into an array of bytes.
pub fn encode_14bit_complex(real: u16, imag: u16) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real & EC_MASK_14BIT) as u32;
    word |= ((imag & EC_MASK_14BIT) as u32) << 14;

    return word.to_le_bytes()
}

/// Encode 2 15-bit real samples into an array of bytes.
pub fn encode_15bit_real(input: [u16; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (input[0] & EC_MASK_15BIT) as u32;
    word |= ((input[1] & EC_MASK_15BIT) as u32) << 15;

    return word.to_le_bytes()
}

/// Encode a 15-bit complex sample into an array of bytes.
pub fn encode_15bit_complex(real: u16, imag: u16) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= (real & EC_MASK_15BIT) as u32;
    word |= ((imag & EC_MASK_15BIT) as u32) << 15;

    return word.to_le_bytes()
}

/// Encode 2 16-bit real samples into an array of bytes.
pub fn encode_16bit_real(input: [u16; 2]) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= input[0] as u32;
    word |= (input[1] as u32) << 16;

    return word.to_le_bytes()
}

/// Encode a 16-bit complex sample into an array of bytes.
pub fn encode_16bit_complex(real: u16, imag: u16) -> [u8; 4] {
    let mut word: u32 = 0;

    word |= real as u32;
    word |= (imag as u32) << 16;

    return word.to_le_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_encode_1bit_real() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u8; 32] = [
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0,
        ];
        assert_eq!(encode_1bit_real(test_in),result)
    }

    #[test]
    fn test_encode_1bit_complex() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: ([u8; 16], [u8; 16]) = (
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(encode_1bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_2bit_real() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(encode_2bit_real(test_in),result)
    }

    #[test]
    fn test_encode_2bit_complex() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: ([u8; 8], [u8; 8]) = ([1, 1, 1, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(encode_2bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_3bit_real() {
        let result: [u8; 4] = (0b00010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u8; 10] = [5, 2, 5, 2, 5, 2, 5, 2, 5, 2];
        assert_eq!(encode_3bit_real(test_in),result)
    }

    #[test]
    fn test_encode_3bit_complex() {
        let result: [u8; 4] = (0b00010101010101010101010101010101_u32).to_le_bytes();
        let test_in: ([u8; 5], [u8; 5]) = ([5, 5, 5, 5, 5], [2, 2, 2, 2, 2]);
        assert_eq!(encode_3bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_4bit_real() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u8; 8] = [5, 5, 5, 5, 5, 5, 5, 5];
        assert_eq!(encode_4bit_real(test_in),result)
    }

    #[test]
    fn test_encode_4bit_complex() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: ([u8; 4], [u8; 4]) = ([5, 5, 5, 5], [5, 5, 5, 5]);
        assert_eq!(encode_4bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_6bit_real() {
        let result: [u8; 4] = (0b00010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u8; 5] = [21, 21, 21, 21, 21];
        assert_eq!(encode_6bit_real(test_in),result)
    }

    #[test]
    fn test_encode_6bit_complex() {
        let result: [u8; 4] = (0b00000000010101010101010101010101_u32).to_le_bytes();
        let test_in: ([u8; 2], [u8; 2]) = ([21, 21], [21, 21]);
        assert_eq!(encode_6bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_7bit_real() {
        let result: [u8; 4] = (0b00000101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u8; 4] = [85, 42, 85, 42];
        assert_eq!(encode_7bit_real(test_in),result)
    }

    #[test]
    fn test_encode_7bit_complex() {
        let result: [u8; 4] = (0b00000101010101010101010101010101_u32).to_le_bytes();
        let test_in: ([u8; 2], [u8; 2]) = ([85, 85], [42, 42]);
        assert_eq!(encode_7bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_8bit_real() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u8; 4] = [85, 85, 85, 85];
        assert_eq!(encode_8bit_real(test_in),result)
    }

    #[test]
    fn test_encode_8bit_complex() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: ([u8; 2], [u8; 2]) = ([85, 85], [85, 85]);
        assert_eq!(encode_8bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_11bit_real() {
        let result: [u8; 4] = (0b00000000000101010101010101010101_u32).to_le_bytes();
        let test_in: [u16; 2] = [0b10101010101, 0b01010101010];
        assert_eq!(encode_11bit_real(test_in),result)
    }

    #[test]
    fn test_encode_11bit_complex() {
        let result: [u8; 4] = (0b00000000000101010101010101010101_u32).to_le_bytes();
        let test_in: (u16, u16) = (0b10101010101, 0b01010101010);
        assert_eq!(encode_11bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_12bit_real() {
        let result: [u8; 4] = (0b00000000010101010101010101010101_u32).to_le_bytes();
        let test_in: [u16; 2] = [0b010101010101, 0b010101010101];
        assert_eq!(encode_12bit_real(test_in),result)
    }

    #[test]
    fn test_encode_12bit_complex() {
        let result: [u8; 4] = (0b00000000010101010101010101010101_u32).to_le_bytes();
        let test_in: (u16, u16) = (0b010101010101, 0b010101010101);
        assert_eq!(encode_12bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_13bit_real() {
        let result: [u8; 4] = (0b00000001010101010101010101010101_u32).to_le_bytes();
        let test_in: [u16; 2] = [0b1010101010101, 0b0101010101010];
        assert_eq!(encode_13bit_real(test_in),result)
    }

    #[test]
    fn test_encode_13bit_complex() {
        let result: [u8; 4] = (0b00000001010101010101010101010101_u32).to_le_bytes();
        let test_in: (u16, u16) = (0b1010101010101, 0b0101010101010);
        assert_eq!(encode_13bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_14bit_real() {
        let result: [u8; 4] = (0b00000101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u16; 2] = [0b01010101010101, 0b01010101010101];
        assert_eq!(encode_14bit_real(test_in),result)
    }

    #[test]
    fn test_encode_14bit_complex() {
        let result: [u8; 4] = (0b00000101010101010101010101010101_u32).to_le_bytes();
        let test_in: (u16, u16) = (0b01010101010101, 0b01010101010101);
        assert_eq!(encode_14bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_15bit_real() {
        let result: [u8; 4] = (0b00010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u16; 2] = [0b101010101010101, 0b010101010101010];
        assert_eq!(encode_15bit_real(test_in),result)
    }

    #[test]
    fn test_encode_15bit_complex() {
        let result: [u8; 4] = (0b00010101010101010101010101010101_u32).to_le_bytes();
        let test_in: (u16, u16) = (0b101010101010101, 0b010101010101010);
        assert_eq!(encode_15bit_complex(test_in.0, test_in.1),result)
    }

    #[test]
    fn test_encode_16bit_real() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: [u16; 2] = [0b0101010101010101, 0b0101010101010101];
        assert_eq!(encode_16bit_real(test_in),result)
    }

    #[test]
    fn test_encode_16bit_complex() {
        let result: [u8; 4] = (0b01010101010101010101010101010101_u32).to_le_bytes();
        let test_in: (u16, u16) = (0b0101010101010101, 0b0101010101010101);
        assert_eq!(encode_16bit_complex(test_in.0, test_in.1),result)
    }
}