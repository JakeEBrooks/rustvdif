//! Functions for encoding VDIF payloads

const EC_MASK_1BIT: u8 = 1;
const EC_MASK_2BIT: u8 = 2u8.pow(2) - 1;
const EC_MASK_3BIT: u8 = 2u8.pow(3) - 1;
const EC_MASK_4BIT: u8 = 2u8.pow(4) - 1;

const EC_MASK_6BIT: u8 = 2u8.pow(6) - 1;
const EC_MASK_7BIT: u8 = 2u8.pow(7) - 1;
const EC_MASK_8BIT: u8 = u8::MAX;

const EC_MASK_11BIT: u16 = 2u16.pow(11) - 1;
const EC_MASK_12BIT: u16 = 2u16.pow(12) - 1;
const EC_MASK_13BIT: u16 = 2u16.pow(13) - 1;
const EC_MASK_14BIT: u16 = 2u16.pow(14) - 1;
const EC_MASK_15BIT: u16 = 2u16.pow(15) - 1;
const EC_MASK_16BIT: u16 = u16::MAX;
const EC_MASK_17BIT: u32 = 2u32.pow(17) - 1;
const EC_MASK_18BIT: u32 = 2u32.pow(18) - 1;
const EC_MASK_19BIT: u32 = 2u32.pow(19) - 1;
const EC_MASK_20BIT: u32 = 2u32.pow(20) - 1;
const EC_MASK_21BIT: u32 = 2u32.pow(21) - 1;
const EC_MASK_22BIT: u32 = 2u32.pow(22) - 1;
const EC_MASK_23BIT: u32 = 2u32.pow(23) - 1;
const EC_MASK_24BIT: u32 = 2u32.pow(24) - 1;
const EC_MASK_25BIT: u32 = 2u32.pow(25) - 1;
const EC_MASK_26BIT: u32 = 2u32.pow(26) - 1;
const EC_MASK_27BIT: u32 = 2u32.pow(27) - 1;
const EC_MASK_28BIT: u32 = 2u32.pow(28) - 1;
const EC_MASK_29BIT: u32 = 2u32.pow(29) - 1;
const EC_MASK_30BIT: u32 = 2u32.pow(30) - 1;
const EC_MASK_31BIT: u32 = 2u32.pow(31) - 1;
const EC_MASK_32BIT: u32 = u32::MAX;

macro_rules! encode_func {
    ($name:ident; $samples:literal; $inty:ty; $mask:ident; $bits:literal) => {
        #[doc = concat!("Encode ", stringify!($samples), " ", stringify!($bits), " bit data samples into a single `u32`.")]
        pub fn $name(input: &[$inty; $samples]) -> u32 {
            let mut outword: u32 = 0;

            for i in 0..$samples {
                outword |= ((input[i] & $mask) as u32) << i*$bits
            }

            return outword
        }
    };
}

macro_rules! encode_func_single {
    ($name:ident; $mask:ident; $bits:literal) => {
        #[doc = concat!("Encode a single ", stringify!($bits), " bit sample into a single `u32`.")]
        pub fn $name(input: &u32) -> u32 {
            return input & $mask
        }
    };
}

encode_func!(encode_1bit; 32; u8; EC_MASK_1BIT; 1);
encode_func!(encode_2bit; 16; u8; EC_MASK_2BIT; 2);
encode_func!(encode_3bit; 10; u8; EC_MASK_3BIT; 3);
encode_func!(encode_4bit; 8; u8; EC_MASK_4BIT; 4);

encode_func!(encode_6bit; 5; u8; EC_MASK_6BIT; 6);
encode_func!(encode_7bit; 4; u8; EC_MASK_7BIT; 7);
encode_func!(encode_8bit; 4; u8; EC_MASK_8BIT; 8);


encode_func!(encode_11bit; 2; u16; EC_MASK_11BIT; 11);
encode_func!(encode_12bit; 2; u16; EC_MASK_12BIT; 12);
encode_func!(encode_13bit; 2; u16; EC_MASK_13BIT; 13);
encode_func!(encode_14bit; 2; u16; EC_MASK_14BIT; 14);
encode_func!(encode_15bit; 2; u16; EC_MASK_15BIT; 15);
encode_func!(encode_16bit; 2; u16; EC_MASK_16BIT; 16);
encode_func_single!(encode_17bit; EC_MASK_17BIT; 17);
encode_func_single!(encode_18bit; EC_MASK_18BIT; 18);
encode_func_single!(encode_19bit; EC_MASK_19BIT; 19);
encode_func_single!(encode_20bit; EC_MASK_20BIT; 20);
encode_func_single!(encode_21bit; EC_MASK_21BIT; 21);
encode_func_single!(encode_22bit; EC_MASK_22BIT; 22);
encode_func_single!(encode_23bit; EC_MASK_23BIT; 23);
encode_func_single!(encode_24bit; EC_MASK_24BIT; 24);
encode_func_single!(encode_25bit; EC_MASK_25BIT; 25);
encode_func_single!(encode_26bit; EC_MASK_26BIT; 26);
encode_func_single!(encode_27bit; EC_MASK_27BIT; 27);
encode_func_single!(encode_28bit; EC_MASK_28BIT; 28);
encode_func_single!(encode_29bit; EC_MASK_29BIT; 29);
encode_func_single!(encode_30bit; EC_MASK_30BIT; 30);
encode_func_single!(encode_31bit; EC_MASK_31BIT; 31);
encode_func_single!(encode_32bit; EC_MASK_32BIT; 32);
