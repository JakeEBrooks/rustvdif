#![warn(missing_docs)]
//! A Rust library for interacting with data encoded in the VLBI Data Interchange Format (VDIF).
//! 
//! Before using this library, it is recommended you familiarize yourself with the VDIF format, if you haven't already,
//! by reading the [VDIF Version 1.1.1 specification](https://vlbi.org/vlbi-standards/vdif/). Put simply, VDIF defines 
//! a 'Data Frame': a datagram-like object consisting of a fixed size header and a payload of bytes.
//! 
//! I take some inspiration from the Python [baseband](https://baseband.readthedocs.io/en/stable/vdif/index.html) library
//! in developing this.

pub mod header;
pub mod frame;
pub mod payload;
pub mod encoding;
pub mod read;
pub mod parsing;