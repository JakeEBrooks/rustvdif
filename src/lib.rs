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
//! # Usage
//! 
//! Reading from VDIF files is quite simple:
//! ```rust,ignore
//! fn main() {
//!     // A file of 8032 byte VDIF frames
//!     let mut file = VDIFReader::open("path/to/my/vdif", 8032).unwrap();
//!     // Read the first 100 frames and print header information on each one
//!     for _ in 0..100 {
//!         let frame = file.read_frame().unwrap();
//!         println!("{}", frame.get_header());
//!     }
//! }
//! ```
//! For writing VDIF frames, see [`VDIFWriter`](crate::io::VDIFWriter).
//! 
//! A [`VDIFReader`](crate::io::VDIFReader) accepts any type implementing [`Read`](std::io::Read), so the pattern is
//! fairly similar even for other data sources. For example a [`TcpStream`](std::net::TcpStream) can be used
//! instead of a [`File`](std::fs::File):
//! ```rust,ignore
//! use std::net::TcpStream;
//! 
//! fn main() {
//!     // Connect to a TCP stream of VDIF frames
//!     let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
//!     // VDIFReader is buffered by default, so use a buffer of 100 frames.
//!     let mut reader = VDIFReader::with_capacity(stream, 8032, 100);
//!     // Read the first 100 frames and print header information on each one
//!     for _ in 0..100 {
//!         let frame = reader.read_frame().unwrap();
//!         println!("{}", frame.get_header());
//!     }
//! }
//! ```
//! 
//! This crate was designed with performance in mind, as is often needed when dealing with high data rate
//! VDIF streams. As such, a minimal amount of memory allocations/copies are performed. Buffered IO is the default for 
//! [`VDIFReader`](crate::io::VDIFReader)s, to minimise expensive system calls.
//! 
//! In general, this library uses byte sizes for the frame size (header *and* payload), and assumes you know the size
//! of the incoming/outgoing VDIF frames in advance.

pub mod frame;
pub mod header;
pub mod data_encoding;
pub mod header_encoding;
pub mod io;
pub mod udp;
pub mod sim;
pub mod vtp;

pub use frame::VDIFFrame;
pub use io::{VDIFReader, VDIFRead, VDIFWriter, VDIFWrite};

// VDIF is an explicitly little endian format. This makes handling it finnicky on big endian targets. A lot of the unsafe
// operations rely on being run on a little endian target and are faster as a result. If a user needs big-endian
// compatibility it is possible, just let me know.

#[cfg(target_endian = "big")]
compile_error!("RustVDIF does not currently support big-endian targets");