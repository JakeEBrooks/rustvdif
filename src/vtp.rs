use std::io::Result;
use std::net::{ToSocketAddrs, UdpSocket};

use crate::VDIFFrame;

/// Reads VDIF frames using the VDIF Transport Protocol (VTP) from a [`UdpSocket`].
/// 
/// The VDIF Transport Protocol is defined [here](https://vlbi.org/wp-content/uploads/2019/03/2012.10.16_VTP_0.9.7.pdf).
///
/// Does not perform any logic or buffering, so all the normal rules and expectations around UDP apply. This implementation 
/// assumes that one datagram consists of a single, complete VDIF frame with an additional 64-bit integer
/// inserted at the start of the datagram.
pub struct VDIFVTP {
    /// The underlying [`UdpSocket`].
    pub sock: UdpSocket,
    frame_size: usize,
}

impl VDIFVTP {
    /// Construct a new [`VDIFVTP`] type attached to a specific socket. Note that `frame_size` is still just the size of the
    /// VDIF frame in bytes.
    pub fn new<A: ToSocketAddrs>(addr: A, frame_size: usize) -> Result<Self> {
        let sock = UdpSocket::bind(addr)?;
        return Ok(Self {
            sock: sock,
            frame_size: frame_size,
        });
    }

    /// [`recv`](std::net::UdpSocket::recv) a [`VDIFFrame`] and the attached `u64` sequence number.
    pub fn recv_frame(&mut self) -> Result<(u64, VDIFFrame)> {
        // Need to get the first u64 from a bunch of u32s. Allocate u64s instead to prevent alignment issues
        // then we can just unsafely reinterpret the rest of the u64s as u32s.
        let mut vtp_frame_buf: Box<[u64]> = vec![0; self.frame_size / 8 + 1].into_boxed_slice();
        let out_frame: VDIFFrame;
        unsafe {
            // Read bytes into vtp_frame_buf
            self.sock.recv(std::slice::from_raw_parts_mut(
                vtp_frame_buf.as_mut_ptr() as *mut u8,
                self.frame_size + 8,
            ))?;
            // Reinterpret all but the first u64 as u32s and copy them to a new VDIF frame.
            out_frame = VDIFFrame::from_slice(std::slice::from_raw_parts(
                (vtp_frame_buf.as_ptr().add(1)) as *const u32,
                self.frame_size / 4,
            ));
        }

        let sequence_number = vtp_frame_buf[0];
        return Ok((sequence_number, out_frame));
    }
}

/// Allows reading VDIF frames from a [`UdpSocket`] in order. Uses the VTP sequence number instead of the VDIF frame number.
///
/// More specifically, [`VDIFOrderedVTP`] implements a simple sequence counting algorithm to ensure that the frame
/// returned by [`next_frame`](VDIFOrderedVTP::next_frame) does not precede the frame previously fetched by the
/// same function.
///
/// For example, say the user has received frame `i` from a call to [`next_frame`](VDIFOrderedVTP::next_frame).
/// Upon calling [`next_frame`](VDIFOrderedVTP::next_frame) again, the value returned is guaranteed to be one of
/// the following:
///
/// - The `i + 1` th frame (most likely).
/// - The `i + n` th frame, where `n` is any *positive* integer.
/// - A duplicate of the `i`th frame.
/// - `None`
///
/// Frames received out of order are simply discarded.
pub struct VDIFOrderedVTP {
    vdifvtp: VDIFVTP,
    expecting_frame: u64,
}

impl VDIFOrderedVTP {
    /// Construct a new [`VDIFOrderedVTP`] type.
    pub fn new<A: ToSocketAddrs>(addr: A, frame_size: usize) -> Result<Self> {
        let vdifvtp = VDIFVTP::new(addr, frame_size)?;
        return Ok(Self {
            vdifvtp: vdifvtp,
            expecting_frame: 0,
        });
    }

    /// Return the next frame in the stream along with its sequence number, or `None` if the frame would be out of order.
    pub fn next_frame(&mut self) -> Result<Option<(u64, VDIFFrame)>> {
        let (seq, in_frame) = self.vdifvtp.recv_frame()?;
        if self.expecting_frame <= seq {
            // Frame is good, increment the expected frame appropriately and
            // return the frame
            self.expecting_frame = seq + 1;
            return Ok(Some((seq, in_frame)));
        } else {
            // Frame is not in order, so just discard it after setting the counter properly.
            self.expecting_frame = seq + 1;
            return Ok(None);
        }
    }

    /// Get a reference to the underlying [`UdpSocket`].
    pub fn socket_ref(&self) -> &UdpSocket {
        return &self.vdifvtp.sock;
    }
}
