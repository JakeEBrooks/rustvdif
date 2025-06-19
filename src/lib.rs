#![warn(missing_docs)]
//! A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF), commonly used in radio astronomy experiments. 
//! The VDIF data format is defined in the VDIF specification, found [here](https://vlbi.org/vlbi-standards/vdif/).
//! 
//! Check out the [examples](./examples) for more information on using this library.


mod frame;
pub use frame::VDIFFrame;
mod header;
pub use header::VDIFHeader;
mod io;
pub use io::{read_frame, write_frame, read_vtp_frame, write_vtp_frame};

pub mod net;
#[allow(missing_docs)]
#[cfg(feature = "recording")]
pub mod recording;

pub mod encoding;
pub mod decoding;

// Don't support big endian targets
#[cfg(target_endian = "big")]
compile_error!("RustVDIF does not currently support big-endian targets");

pub(crate) mod header_masks {
    pub(crate) const MASK_IS_VALID: u32 = 0x80000000;
    pub(crate) const MASK_IS_LEGACY: u32 = 0x40000000;
    pub(crate) const MASK_TIME: u32 = 0x3FFFFFFF;
    pub(crate) const MASK_REF_EPOCH: u32 = 0x3F000000;
    pub(crate) const MASK_FRAME_NO: u32 = 0x00FFFFFF;
    pub(crate) const MASK_VERSION_NO: u32 = 0xE0000000;
    pub(crate) const MASK_LOG2_CHANNELS: u32 = 0x1F000000;
    pub(crate) const MASK_SIZE8: u32 = 0x00FFFFFF;
    pub(crate) const MASK_IS_REAL: u32 = 0x80000000;
    pub(crate) const MASK_BITS_PER_SAMPLE: u32 = 0x7C000000;
    pub(crate) const MASK_THREAD_ID: u32 = 0x03FF0000;
    pub(crate) const MASK_STATION_ID: u32 = 0x0000FFFF;
}