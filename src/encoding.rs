//! Provides functionality for encoding/decoding VDIF payloads.

// Other VDIF software uses a LUT for decoding the u32 word, but
// writing it out as below seems to be ~2x faster. This is tested
// for 1 bit decoding, I assume it holds for higher bit depths.

// The decoding functions are quite ugly, but writing it out explicitly
// maximises information to the compiler and doesn't run the risk of
// loops not being unrolled

const MASK_1BIT: u32 = 0b00000000000000000000000000000001;
const MASK_2BIT: u32 = 0b00000000000000000000000000000011;
const MASK_3BIT: u32 = 0b00000000000000000000000000000111;
const MASK_4BIT: u32 = 0b00000000000000000000000000001111;

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

    return out
}

/// Decode a VDIF encoded 32-bit word of 1-bit complex samples.
pub fn decode_1bit_complex(input: &u32) -> ([u8; 16], [u8; 16]) {
    let mut ip_out: [u8; 16] = [0; 16];
    let mut q_out: [u8; 16] = [0; 16];

    ip_out[0]   = (input & MASK_1BIT) as u8;
    q_out[0]    = ((input >> 1) & MASK_1BIT) as u8;
    ip_out[1]   = ((input >> 2) & MASK_1BIT) as u8;
    q_out[1]    = ((input >> 3) & MASK_1BIT) as u8;
    ip_out[2]   = ((input >> 4) & MASK_1BIT) as u8;
    q_out[2]    = ((input >> 5) & MASK_1BIT) as u8;
    ip_out[3]   = ((input >> 6) & MASK_1BIT) as u8;
    q_out[3]    = ((input >> 7) & MASK_1BIT) as u8;
    ip_out[4]   = ((input >> 8) & MASK_1BIT) as u8;
    q_out[4]    = ((input >> 9) & MASK_1BIT) as u8;
    ip_out[5]   = ((input >> 10) & MASK_1BIT) as u8;
    q_out[5]    = ((input >> 11) & MASK_1BIT) as u8;
    ip_out[6]   = ((input >> 12) & MASK_1BIT) as u8;
    q_out[6]    = ((input >> 13) & MASK_1BIT) as u8;
    ip_out[7]   = ((input >> 14) & MASK_1BIT) as u8;
    q_out[7]    = ((input >> 15) & MASK_1BIT) as u8;
    ip_out[8]   = ((input >> 16) & MASK_1BIT) as u8;
    q_out[8]    = ((input >> 17) & MASK_1BIT) as u8;
    ip_out[9]   = ((input >> 18) & MASK_1BIT) as u8;
    q_out[9]    = ((input >> 19) & MASK_1BIT) as u8;
    ip_out[10]  = ((input >> 20) & MASK_1BIT) as u8;
    q_out[10]   = ((input >> 21) & MASK_1BIT) as u8;
    ip_out[11]  = ((input >> 22) & MASK_1BIT) as u8;
    q_out[11]   = ((input >> 23) & MASK_1BIT) as u8;
    ip_out[12]  = ((input >> 24) & MASK_1BIT) as u8;
    q_out[12]   = ((input >> 25) & MASK_1BIT) as u8;
    ip_out[13]  = ((input >> 26) & MASK_1BIT) as u8;
    q_out[13]   = ((input >> 27) & MASK_1BIT) as u8;
    ip_out[14]  = ((input >> 28) & MASK_1BIT) as u8;
    q_out[14]   = ((input >> 29) & MASK_1BIT) as u8;
    ip_out[15]  = ((input >> 30) & MASK_1BIT) as u8;
    q_out[15]   = ((input >> 31) & MASK_1BIT) as u8;

    return (ip_out, q_out)
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

    return out
}

/// Decode a VDIF encoded 32-bit word of 2-bit complex samples.
pub fn decode_2bit_complex(input: &u32) -> ([u8; 8], [u8; 8]) {
    let mut ip_out: [u8; 8] = [0; 8];
    let mut q_out: [u8; 8] = [0; 8];

    ip_out[0]   = (input & MASK_2BIT) as u8;
    q_out[0]    = ((input >> 2) & MASK_2BIT) as u8;
    ip_out[1]   = ((input >> 4) & MASK_2BIT) as u8;
    q_out[1]    = ((input >> 6) & MASK_2BIT) as u8;
    ip_out[2]   = ((input >> 8) & MASK_2BIT) as u8;
    q_out[2]    = ((input >> 10) & MASK_2BIT) as u8;
    ip_out[3]   = ((input >> 12) & MASK_2BIT) as u8;
    q_out[3]    = ((input >> 14) & MASK_2BIT) as u8;
    ip_out[4]   = ((input >> 16) & MASK_2BIT) as u8;
    q_out[4]    = ((input >> 18) & MASK_2BIT) as u8;
    ip_out[5]   = ((input >> 20) & MASK_2BIT) as u8;
    q_out[5]    = ((input >> 22) & MASK_2BIT) as u8;
    ip_out[6]   = ((input >> 24) & MASK_2BIT) as u8;
    q_out[6]    = ((input >> 26) & MASK_2BIT) as u8;
    ip_out[7]   = ((input >> 28) & MASK_2BIT) as u8;
    q_out[7]    = ((input >> 30) & MASK_2BIT) as u8;

    return (ip_out, q_out)
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

    return out
}

/// Decode a VDIF encoded 32-bit word of 3-bit complex samples.
pub fn decode_3bit_complex(input: &u32) -> ([u8; 5], [u8; 5]) {
    let mut ip_out: [u8; 5] = [0; 5];
    let mut q_out: [u8; 5] = [0; 5];

    ip_out[0]   = (input & MASK_3BIT) as u8;
    q_out[0]    = ((input >> 3) & MASK_3BIT) as u8;
    ip_out[1]   = ((input >> 6) & MASK_3BIT) as u8;
    q_out[1]    = ((input >> 9) & MASK_3BIT) as u8;
    ip_out[2]   = ((input >> 12) & MASK_3BIT) as u8;
    q_out[2]    = ((input >> 15) & MASK_3BIT) as u8;
    ip_out[3]   = ((input >> 18) & MASK_3BIT) as u8;
    q_out[3]    = ((input >> 21) & MASK_3BIT) as u8;
    ip_out[4]   = ((input >> 24) & MASK_3BIT) as u8;
    q_out[4]    = ((input >> 27) & MASK_3BIT) as u8;

    return (ip_out, q_out)
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

    return out
}

/// Decode a VDIF encoded 32-bit word of 4-bit complex samples.
pub fn decode_4bit_complex(input: &u32) -> ([u8; 4], [u8; 4]) {
    let mut ip_out: [u8; 4] = [0; 4];
    let mut q_out: [u8; 4] = [0; 4];

    ip_out[0]   = (input & MASK_4BIT) as u8;
    q_out[0]    = ((input >> 4) & MASK_4BIT) as u8;
    ip_out[1]   = ((input >> 8) & MASK_4BIT) as u8;
    q_out[1]    = ((input >> 12) & MASK_4BIT) as u8;
    ip_out[2]   = ((input >> 16) & MASK_4BIT) as u8;
    q_out[2]    = ((input >> 20) & MASK_4BIT) as u8;
    ip_out[3]   = ((input >> 24) & MASK_4BIT) as u8;
    q_out[3]    = ((input >> 28) & MASK_4BIT) as u8;

    return (ip_out, q_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_1bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 32] = [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0];
        assert_eq!(decode_1bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_1bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 16], [u8; 16]) = ([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        assert_eq!(decode_1bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_2bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 16] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
        assert_eq!(decode_2bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_2bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 8], [u8; 8]) = ([1,1,1,1,1,1,1,1], [1,1,1,1,1,1,1,1]);
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
        let result: ([u8; 5], [u8; 5]) = ([5,5,5,5,5], [2,2,2,2,2]);
        assert_eq!(decode_3bit_complex(&test_in), result)
    }

    #[test]
    fn test_decode_4bit_real() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: [u8; 8] = [5,5,5,5,5,5,5,5];
        assert_eq!(decode_4bit_real(&test_in), result)
    }

    #[test]
    fn test_decode_4bit_complex() {
        let test_in: u32 = 0b01010101010101010101010101010101;
        let result: ([u8; 4], [u8; 4]) = ([5,5,5,5], [5,5,5,5]);
        assert_eq!(decode_4bit_complex(&test_in), result)
    }
}