#![warn(missing_docs)]
//! A Rust library for interacting with data encoded in the VLBI Data Interchange Format (VDIF).
//!
//! Before using this library, it is recommended you familiarize yourself with the VDIF format, if you haven't already,
//! by reading the [VDIF Version 1.1.1 specification](https://vlbi.org/vlbi-standards/vdif/). Put simply, VDIF defines
//! a 'Data Frame': a datagram-like object consisting of a fixed size header and a payload of bytes.
//!
//! I take some inspiration from the Python [baseband](https://baseband.readthedocs.io/en/stable/vdif/index.html) library
//! in developing this.
//! 
//! # Getting Started
//! 
//! If you're working with files, you'll want to check out [`VDIFFileReader`](crate::read::VDIFFileReader), which allows
//! you to read [`VDIFFrame`](crate::frame::VDIFFrame)s from a file like so:
//! 
//! ```rust,no_run
//! use rustvdif::VDIFFileReader;
//! 
//! fn main() {
//!     let mut file = VDIFFileReader::open("path/to/my/vdif/file").unwrap();
//!     // Read the first frame in the file
//!     let frame0 = file.get_frame().unwrap();
//!     println!("{}", frame0);
//! }
//! ```
//! 
//! You can then read the next frame by calling [`get_frame`](crate::read::VDIFFileReader::get_frame) again, or skip the next
//! frame by calling [`nextframe`](crate::read::VDIFFileReader::nextframe). If you want to read all frames from the file
//! (be careful with big files!), you can call [`get_all_frames`](crate::read::VDIFFileReader::get_all_frames).
//! 
//! If your working with VDIF data from other sources you'll be using the more general [`VDIFReader`](crate::read::VDIFReader)
//! type, which allows you to wrap any type implementing [`std::io::Read`].
//! 
//! For decoding the payload, check out the [`encoding`] module.

pub mod encoding;
pub mod frame;
pub mod header;
pub mod parsing;
pub mod payload;
pub mod read;

pub use read::{VDIFFileReader, VDIFReader};
pub use frame::VDIFFrame;
pub use payload::VDIFPayload;
pub use header::VDIFHeader;
