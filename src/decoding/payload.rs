//! Functions for decoding VDIF payloads
//!
//! Note that these functions *may* not be the most performant way of doing what you need, but are provided for
//! convenience, or for when you just want to inspect a VDIF frame's payload.
//!
//! While this library supports uncommon bits per sample like 6 bit, you should try to stick to 2^n bits per sample
//! (i.e. 1, 2, 4, 8, 16, 32) since they are more efficient to store in VDIF.

// Other VDIF software uses a LUT for decoding the u32 word, but
// writing it out as below seems to be at least the same speed, if not faster.
// This is tested for 1 bit decoding, I assume it holds for higher bit depths since
// they require less operations than 1 bit. Though I guess a user may want to map
// the samples to something else anyway

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
const DC_MASK_17BIT: u32 = u32::MAX >> 15;
const DC_MASK_18BIT: u32 = u32::MAX >> 14;
const DC_MASK_19BIT: u32 = u32::MAX >> 13;
const DC_MASK_20BIT: u32 = u32::MAX >> 12;
const DC_MASK_21BIT: u32 = u32::MAX >> 11;
const DC_MASK_22BIT: u32 = u32::MAX >> 10;
const DC_MASK_23BIT: u32 = u32::MAX >> 9;
const DC_MASK_24BIT: u32 = u32::MAX >> 8;
const DC_MASK_25BIT: u32 = u32::MAX >> 7;
const DC_MASK_26BIT: u32 = u32::MAX >> 6;
const DC_MASK_27BIT: u32 = u32::MAX >> 5;
const DC_MASK_28BIT: u32 = u32::MAX >> 4;
const DC_MASK_29BIT: u32 = u32::MAX >> 3;
const DC_MASK_30BIT: u32 = u32::MAX >> 2;
const DC_MASK_31BIT: u32 = u32::MAX >> 1;
const DC_MASK_32BIT: u32 = u32::MAX >> 0;

macro_rules! decode_func {
    ($name:ident; $samples:literal; $outty:ty; $mask:ident; $bits:literal) => {
        #[doc = concat!("Decode a VDIF encoded `u32` into ", stringify!($samples), " ", stringify!($bits), " bit samples")]
        pub fn $name(input: &u32) -> [$outty; $samples] {
            let mut out: [$outty; $samples] = [0; $samples];

            for i in 0..$samples {
                out[i] = ((input >> i*$bits) & $mask) as $outty;
            }

            return out
        }
    };
}

macro_rules! decode_func_single {
    ($name:ident; $mask:ident; $bits:literal) => {
        #[doc = concat!("Decode a VDIF encoded `u32` into one ", stringify!($bits), " bit sample")]
        pub fn $name(input: &u32) -> u32 {
            return input & $mask
        }
    };
}

decode_func!(decode_1bit; 32; u8; DC_MASK_1BIT; 1);
decode_func!(decode_2bit; 16; u8; DC_MASK_2BIT; 2);
decode_func!(decode_3bit; 10; u8; DC_MASK_3BIT; 3);
decode_func!(decode_4bit; 8; u8; DC_MASK_4BIT; 4);

decode_func!(decode_6bit; 5; u8; DC_MASK_6BIT; 6);
decode_func!(decode_7bit; 4; u8; DC_MASK_7BIT; 7);
decode_func!(decode_8bit; 4; u8; DC_MASK_8BIT; 8);


decode_func!(decode_11bit; 2; u16; DC_MASK_11BIT; 11);
decode_func!(decode_12bit; 2; u16; DC_MASK_12BIT; 12);
decode_func!(decode_13bit; 2; u16; DC_MASK_13BIT; 13);
decode_func!(decode_14bit; 2; u16; DC_MASK_14BIT; 14);
decode_func!(decode_15bit; 2; u16; DC_MASK_15BIT; 15);
decode_func!(decode_16bit; 2; u16; DC_MASK_16BIT; 16);
decode_func_single!(decode_17bit; DC_MASK_17BIT; 17);
decode_func_single!(decode_18bit; DC_MASK_18BIT; 18);
decode_func_single!(decode_19bit; DC_MASK_19BIT; 19);
decode_func_single!(decode_20bit; DC_MASK_20BIT; 20);
decode_func_single!(decode_21bit; DC_MASK_21BIT; 21);
decode_func_single!(decode_22bit; DC_MASK_22BIT; 22);
decode_func_single!(decode_23bit; DC_MASK_23BIT; 23);
decode_func_single!(decode_24bit; DC_MASK_24BIT; 24);
decode_func_single!(decode_25bit; DC_MASK_25BIT; 25);
decode_func_single!(decode_26bit; DC_MASK_26BIT; 26);
decode_func_single!(decode_27bit; DC_MASK_27BIT; 27);
decode_func_single!(decode_28bit; DC_MASK_28BIT; 28);
decode_func_single!(decode_29bit; DC_MASK_29BIT; 29);
decode_func_single!(decode_30bit; DC_MASK_30BIT; 30);
decode_func_single!(decode_31bit; DC_MASK_31BIT; 31);
decode_func_single!(decode_32bit; DC_MASK_32BIT; 32);
