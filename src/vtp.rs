use std::io::Result;
use std::net::{ToSocketAddrs, UdpSocket};
use std::os::fd::{AsFd, AsRawFd, FromRawFd, IntoRawFd};

use crate::{VDIFFrame, VDIFRead};

/// Reads VDIF frames encoded using the VDIF Transport Protocol (VTP) from a [`UdpSocket`].
///
/// The VDIF Transport Protocol is defined [here](https://vlbi.org/wp-content/uploads/2019/03/2012.10.16_VTP_0.9.7.pdf).
///
/// Does not perform any logic or buffering, so all the normal rules and expectations around UDP apply. This implementation
/// assumes that one datagram consists of a single, complete VDIF frame with an additional 64-bit integer
/// inserted at the start of the datagram.
pub struct VDIFVTP {
    sock: UdpSocket,
    frame_size: usize,
}

impl VDIFVTP {
    /// Construct a new [`VDIFVTP`] type attached to a specific socket.
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let sock = UdpSocket::bind(addr)?;
        return Ok(Self {
            sock: sock,
            frame_size: crate::DEFAULT_FRAMESIZE,
        });
    }

    /// Set the expected frame_size.
    pub fn set_framesize(&mut self, frame_size: usize) {
        self.frame_size = frame_size
    }

    /// Read a [`VDIFFrame`] and the attached `u64` sequence number.
    pub fn read_frame_withseq(&mut self) -> Result<(u64, VDIFFrame)> {
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

    /// Write a [`VDIFFrame`] with an attached `u64` sequence number.
    pub fn write_frame_withseq(&mut self, seq: u64, frame: &VDIFFrame) -> Result<()> {
        // Create a buffer to store the full datagram, initially as u64s
        let mut vtp_frame_buf: Box<[u64]> = vec![0; frame.bytesize() / 8 + 1].into_boxed_slice();
        // Set the first u64 as the sequence number
        vtp_frame_buf[0] = seq;
        // Unsafely copy the contents of the input frame to the remainder of the buffer
        unsafe {
            // The VDIFFrame part of vtp_frame_buf as a mutable slice
            let vtp_frame_buf_u32 = std::slice::from_raw_parts_mut(
                vtp_frame_buf.as_mut_ptr().add(1) as *mut u32,
                (vtp_frame_buf.len() - 1) * 2,
            );
            vtp_frame_buf_u32.copy_from_slice(frame.as_slice());
            // Send it on as bytes
            self.sock.send(std::slice::from_raw_parts(
                vtp_frame_buf.as_ptr() as *const u8,
                vtp_frame_buf.len() * 8,
            ))?;
        }

        return Ok(());
    }

    /// Return a reference to the underlying [`UdpSocket`].
    pub fn get_ref(&self) -> &UdpSocket {
        return &self.sock;
    }

    /// Return a mutable reference to the underlying [`UdpSocket`].
    pub fn get_mut(&mut self) -> &mut UdpSocket {
        return &mut self.sock;
    }

    /// Consume self and return the underlying [`UdpSocket`].
    pub fn into_inner(self) -> UdpSocket {
        return self.sock;
    }
}

impl VDIFRead for VDIFVTP {
    fn read_frame(&mut self) -> Result<VDIFFrame> {
        let (_, frame) = self.read_frame_withseq()?;
        return Ok(frame);
    }
}

impl AsFd for VDIFVTP {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        return self.sock.as_fd()
    }
}

impl AsRawFd for VDIFVTP {
    fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
        return self.sock.as_raw_fd()
    }
}

impl FromRawFd for VDIFVTP {
    unsafe fn from_raw_fd(fd: std::os::unix::prelude::RawFd) -> Self {
        return Self { sock: UdpSocket::from_raw_fd(fd), frame_size: crate::DEFAULT_FRAMESIZE }
    }
}

impl IntoRawFd for VDIFVTP {
    fn into_raw_fd(self) -> std::os::unix::prelude::RawFd {
        return self.sock.into_raw_fd()
    }
}

impl From<UdpSocket> for VDIFVTP {
    fn from(value: UdpSocket) -> Self {
        return Self { sock: value, frame_size: crate::DEFAULT_FRAMESIZE }
    }
}

// TODO: Implement VDIFWrite for VDIFVTP? But how to handle the sequence number without user control?
