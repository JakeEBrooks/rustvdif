//! Functionality for sending/receiving VDIF data over a network

mod udp;
mod vtp;

#[doc(inline)]
pub use udp::*;

#[doc(inline)]
pub use vtp::*;