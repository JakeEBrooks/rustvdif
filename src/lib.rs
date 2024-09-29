#![warn(missing_docs)]
//! A Rust library for interacting with data encoded in the VLBI Data Interchange Format (VDIF).
//! 
//! Before using this library, it is recommended you familiarize yourself with the VDIF format, if you haven't already,
//! by reading the [VDIF Version 1.1.1 specification](https://vlbi.org/vlbi-standards/vdif/). Put simply, VDIF defines 
//! a 'Data Frame': a datagram-like object consisting of a fixed size [header](VDIFHeader) 
//! and a payload of bytes.
//! 
//! This library enables you to easily interact with a wide array of VDIF data sources through 
//! the [`VDIFReader`](read::VDIFReader) object. Legacy VDIF is not currently supported, but this may change if there
//! is interest.
//! 
//! I take some inspiration from the Python [baseband](https://baseband.readthedocs.io/en/stable/vdif/index.html) library
//! in developing this.

pub mod header;
pub mod read;
pub mod file;

use header::{VDIFHeader, VDIF_HEADER_SIZE};

/// A VDIF Data Frame.
/// 
/// Contains a [`VDIFHeader`] and [`Box`] containing a raw [`[u8]`](std::slice) byte-slice.
pub struct VDIFDataFrame {
    header: VDIFHeader,
    payload: Box<[u8]>
}

impl VDIFDataFrame {
    /// Construct a new [`VDIFDataFrame`] from a [`VDIFHeader`] and a byte slice.
    pub fn new(header: VDIFHeader, payload: Box<[u8]>) -> Self {
        return Self{header: header, payload: payload}
    }

    /// Return the total size of this data frame, including the header and payload.
    pub fn byte_size(&self) -> u32 {
        return self.header.byte_size()
    }

    /// Returns a reference to the [`VDIFHeader`] owned by this data frame.
    pub fn get_header(&self) -> &VDIFHeader {
        return &self.header
    }

    /// Returns a reference to a byte array representing the [`VDIFHeader`] of this data frame.
    pub fn get_header_data(&self) -> &[u8; VDIF_HEADER_SIZE] {
        return self.header.get_ref()
    }

    /// Returns a reference to the byte slice representing the payload of this data frame.
    pub fn get_payload(&self) -> &[u8] {
        return &self.payload
    }

    /// Consume `self` and transfer ownership of its components out of this [`VDIFDataFrame`] to the user.
    pub fn take(self) -> (VDIFHeader, Box<[u8]>) {
        return (self.header, self.payload)
    }
}