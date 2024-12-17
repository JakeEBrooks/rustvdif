//! Types and methods to assist in sending and receiving VDIF frames using UDP.
//!
//! This implementation assumes that one datagram consists of a single, complete VDIF frame.

use std::io::Result;
use std::net::{ToSocketAddrs, UdpSocket};

use crate::header_encoding::MASK_FRAME_NO;
use crate::VDIFFrame;

/// A simple wrapper around a [`UdpSocket`] to [`recv`](std::net::UdpSocket::recv) frames.
///
/// Does not perform any logic or buffering, so all the normal rules and expectations around UDP apply.
pub struct VDIFUDP {
    /// The underlying [`UdpSocket`].
    pub sock: UdpSocket,
    frame_size: usize,
}

impl VDIFUDP {
    /// Construct a new [`VDIFUDP`] type attached to a specific socket.
    pub fn new<A: ToSocketAddrs>(addr: A, frame_size: usize) -> Result<Self> {
        let sock = UdpSocket::bind(addr)?;
        return Ok(Self {
            sock: sock,
            frame_size: frame_size,
        });
    }

    /// [`recv`](std::net::UdpSocket::recv) a [`VDIFFrame`].
    pub fn recv_frame(&mut self) -> Result<VDIFFrame> {
        let mut frame = VDIFFrame::empty(self.frame_size);
        self.sock.recv(frame.as_mut_bytes())?;
        return Ok(frame);
    }

    /// [`send`](std::net::UdpSocket::send) a [`VDIFFrame`].
    pub fn send_frame(&mut self, frame: VDIFFrame) -> Result<()> {
        let _ = self.sock.send(frame.as_bytes())?;
        return Ok(());
    }
}

/// Allows reading VDIF frames in order.
///
/// More specifically, [`VDIFOrderedUDP`] implements a simple sequence counting algorithm to ensure that the frame
/// returned by [`next_frame`](VDIFOrderedUDP::next_frame) does not precede the frame previously fetched by the
/// same function.
///
/// For example, say the user has received frame `i` from a call to [`next_frame`](VDIFOrderedUDP::next_frame).
/// Upon calling [`next_frame`](VDIFOrderedUDP::next_frame) again, the value returned is guaranteed to be one of
/// the following:
///
/// - The `i + 1` th frame (most likely).
/// - The `i + n` th frame, where `n` is any *positive* integer.
/// - A duplicate of the `i`th frame.
/// - `None`
///
/// Frames received out of order are simply discarded.
pub struct VDIFOrderedUDP {
    vdifudp: VDIFUDP,

    frame_rate: u32,
    expecting_frame: u32,
}

impl VDIFOrderedUDP {
    /// Construct a new [`VDIFOrderedUDP`] type. Note `frame_rate` is the the number of frames contained within one second *per* thread.
    pub fn new<A: ToSocketAddrs>(addr: A, frame_size: usize, frame_rate: u32) -> Result<Self> {
        let vdifudp = VDIFUDP::new(addr, frame_size)?;
        return Ok(Self {
            vdifudp: vdifudp,
            frame_rate: frame_rate,
            expecting_frame: 0,
        });
    }

    /// Return the next frame in the stream, or `None` if the frame would be out of order.
    pub fn next_frame(&mut self) -> Result<Option<VDIFFrame>> {
        let in_frame = self.vdifudp.recv_frame()?;
        let in_frame_no = check_frame_no(&in_frame);
        if self.expecting_frame <= in_frame_no {
            // Frame is good, increment the expected frame appropriately and
            // return the frame
            self.expecting_frame = if self.expecting_frame < self.frame_rate {
                in_frame_no + 1
            } else {
                0
            };
            return Ok(Some(in_frame));
        } else {
            // Frame is not in order, so just discard it after setting the counter properly.
            self.expecting_frame = if self.expecting_frame < self.frame_rate {
                in_frame_no + 1
            } else {
                0
            };

            return Ok(None);
        }
    }

    /// Get a reference to the underlying [`UdpSocket`].
    pub fn socket_ref(&self) -> &UdpSocket {
        return &self.vdifudp.sock;
    }
}

/// Quickly check the frame number without decoding the whole header
fn check_frame_no(frame: &VDIFFrame) -> u32 {
    return frame.get_word(1) & MASK_FRAME_NO;
}
