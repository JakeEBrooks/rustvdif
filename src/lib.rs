#![warn(missing_docs)]

//! A rust crate for interacting with data encoded in the VLBI Data Interchange Format (VDIF), commonly used in
//! radio astronomy experiments. The VDIF data format is defined in the VDIF specification,
//! found [here](https://vlbi.org/vlbi-standards/vdif/).
//!
//! This is a minimalist crate designed to relieve the problem of dealing with VDIF data in your own applications.
//!
//! With `rustvdif` you can:
//!
//! - Read VDIF frames from and write to various sources, including files, TCP Streams and UDP Sockets.
//! - Access VDIF data encoded using the VDIF Transport Protocol (VTP)
//! - Easily access fields within a VDIF header.
//! - Access VDIF payload data in `u32` or byte form.
//! - Encode and decode VDIF payloads, with up to 16 bits/sample.
//!
//! In general, this library uses byte sizes for the frame size (the payload size plus 32), and assumes you know the size
//! of the incoming/outgoing VDIF frames in advance.

mod frame;
pub mod payloadencoding;
pub use frame::VDIFFrame;
mod header;
pub use header::*;
pub mod headerencoding;
mod io;
pub use io::*;
mod udp;
pub use udp::*;
mod vtp;
pub use vtp::*;

pub(crate) const DEFAULT_FRAMESIZE: usize = 5032;

// VDIF is an explicitly little endian format. This makes handling it finnicky on big endian targets. A lot of the unsafe
// operations rely on being run on a little endian target and are faster as a result. If a user needs big-endian
// compatibility it is possible, just let me know.

#[cfg(target_endian = "big")]
compile_error!("RustVDIF does not currently support big-endian targets");
