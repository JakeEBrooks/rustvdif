use rustvdif::decoding::header::*;
use rustvdif::encoding::header::*;
use rustvdif::VDIFFrame;

use rustvdif::decoding::payload::*;
use rustvdif::encoding::payload::*;

#[test]
fn test_encode_is_valid() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_is_valid(&mut test_frame, true);
    assert_eq!(true, decode_is_valid(&test_frame))
}

#[test]
fn test_encode_is_legacy() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_is_legacy(&mut test_frame, false);
    assert_eq!(false, decode_is_legacy(&test_frame))
}

#[test]
fn test_encode_time() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_time(&mut test_frame, 12345);
    assert_eq!(12345, decode_time(&test_frame))
}

#[test]
fn test_encode_ref_epoch() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_ref_epoch(&mut test_frame, 2);
    assert_eq!(2, decode_ref_epoch(&test_frame))
}

#[test]
fn test_encode_frameno() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_frameno(&mut test_frame, 4001);
    assert_eq!(4001, decode_frameno(&test_frame))
}

#[test]
fn test_encode_version() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_version(&mut test_frame, 2);
    assert_eq!(2, decode_version(&test_frame))
}

#[test]
fn test_encode_log2channels() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_log2channels(&mut test_frame, 3);
    assert_eq!(3, decode_log2channels(&test_frame))
}

#[test]
fn test_encode_size8() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_size8(&mut test_frame, 1032/8);
    assert_eq!(1032/8, decode_size8(&test_frame))
}

#[test]
fn test_encode_is_real() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_is_real(&mut test_frame, true);
    assert_eq!(true, decode_is_real(&test_frame))
}

#[test]
fn test_encode_bits_per_sample_1() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_bits_per_sample_1(&mut test_frame, 8);
    assert_eq!(8, decode_bits_per_sample_1(&test_frame))
}

#[test]
fn test_encode_threadid() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_threadid(&mut test_frame, 5);
    assert_eq!(5, decode_threadid(&test_frame))
}

#[test]
fn test_encode_stationid() {
    let mut test_frame = VDIFFrame::new_empty(1032);
    encode_stationid(&mut test_frame, 42);
    assert_eq!(42, decode_stationid(&test_frame))
}

const TEST_DATA_WORD: u32 = u32::MAX;

macro_rules! test_encode_func {
    ($name:ident; $enc:ident; $dec:ident; $res:expr) => {
        #[test]
        fn $name() {
            assert_eq!($res, $enc(&$dec(&TEST_DATA_WORD)))
        }
    };
}

test_encode_func!(test_encode_1bit_data; encode_1bit; decode_1bit; u32::MAX);
test_encode_func!(test_encode_2bit_data; encode_2bit; decode_2bit; u32::MAX);
test_encode_func!(test_encode_3bit_data; encode_3bit; decode_3bit; 0x3FFFFFFF);
test_encode_func!(test_encode_4bit_data; encode_4bit; decode_4bit; u32::MAX);
test_encode_func!(test_encode_6bit_data; encode_6bit; decode_6bit; 0x3FFFFFFF);
test_encode_func!(test_encode_7bit_data; encode_7bit; decode_7bit; 0x0FFFFFFF);
test_encode_func!(test_encode_8bit_data; encode_8bit; decode_8bit; u32::MAX);
test_encode_func!(test_encode_11bit_data; encode_11bit; decode_11bit; 0x003FFFFF);
test_encode_func!(test_encode_12bit_data; encode_12bit; decode_12bit; 0x00FFFFFF);
test_encode_func!(test_encode_13bit_data; encode_13bit; decode_13bit; 0x03FFFFFF);
test_encode_func!(test_encode_14bit_data; encode_14bit; decode_14bit; 0x0FFFFFFF);
test_encode_func!(test_encode_15bit_data; encode_15bit; decode_15bit; 0x3FFFFFFF);
test_encode_func!(test_encode_16bit_data; encode_16bit; decode_16bit; u32::MAX);
test_encode_func!(test_encode_17bit_data; encode_17bit; decode_17bit; 0x0001FFFF);
test_encode_func!(test_encode_18bit_data; encode_18bit; decode_18bit; 0x0003FFFF);
test_encode_func!(test_encode_19bit_data; encode_19bit; decode_19bit; 0x0007FFFF);
test_encode_func!(test_encode_20bit_data; encode_20bit; decode_20bit; 0x000FFFFF);
test_encode_func!(test_encode_21bit_data; encode_21bit; decode_21bit; 0x001FFFFF);
test_encode_func!(test_encode_22bit_data; encode_22bit; decode_22bit; 0x003FFFFF);
test_encode_func!(test_encode_23bit_data; encode_23bit; decode_23bit; 0x007FFFFF);
test_encode_func!(test_encode_24bit_data; encode_24bit; decode_24bit; 0x00FFFFFF);
test_encode_func!(test_encode_25bit_data; encode_25bit; decode_25bit; 0x01FFFFFF);
test_encode_func!(test_encode_26bit_data; encode_26bit; decode_26bit; 0x03FFFFFF);
test_encode_func!(test_encode_27bit_data; encode_27bit; decode_27bit; 0x07FFFFFF);
test_encode_func!(test_encode_28bit_data; encode_28bit; decode_28bit; 0x0FFFFFFF);
test_encode_func!(test_encode_29bit_data; encode_29bit; decode_29bit; 0x1FFFFFFF);
test_encode_func!(test_encode_30bit_data; encode_30bit; decode_30bit; 0x3FFFFFFF);
test_encode_func!(test_encode_31bit_data; encode_31bit; decode_31bit; 0x7FFFFFFF);
test_encode_func!(test_encode_32bit_data; encode_32bit; decode_32bit; 0xFFFFFFFF);