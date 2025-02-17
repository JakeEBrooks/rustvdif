use std::io::Result;
use std::net::{ToSocketAddrs, UdpSocket};
use std::os::fd::{AsFd, AsRawFd, FromRawFd, IntoRawFd};

use crate::{VDIFFrame, VDIFRead, VDIFWrite};

/// Reads VDIF frames from a [`UdpSocket`].
///
/// Does not perform any logic or buffering, so all the normal rules and expectations around UDP apply. This implementation
/// assumes that one datagram consists of a single, complete VDIF frame. The default framesize is set to 5032, but can be changed using
/// [`set_framesize`](VDIFUDP::set_framesize).
pub struct VDIFUDP {
    sock: UdpSocket,
    frame_size: usize,
}

impl VDIFUDP {
    /// Construct a new [`VDIFUDP`] type attached to a specific socket.
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

impl VDIFRead for VDIFUDP {
    fn read_frame(&mut self) -> Result<VDIFFrame> {
        let mut frame = VDIFFrame::empty(self.frame_size);
        self.sock.recv(frame.as_mut_bytes())?;
        return Ok(frame);
    }
}

impl VDIFWrite for VDIFUDP {
    fn write_frame(&mut self, frame: &VDIFFrame) -> Result<()> {
        let _ = self.sock.send(frame.as_bytes())?;
        return Ok(());
    }
}

impl AsFd for VDIFUDP {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        return self.sock.as_fd()
    }
}

impl AsRawFd for VDIFUDP {
    fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
        return self.sock.as_raw_fd()
    }
}

impl FromRawFd for VDIFUDP {
    unsafe fn from_raw_fd(fd: std::os::unix::prelude::RawFd) -> Self {
        return Self { sock: UdpSocket::from_raw_fd(fd), frame_size: crate::DEFAULT_FRAMESIZE }
    }
}

impl IntoRawFd for VDIFUDP {
    fn into_raw_fd(self) -> std::os::unix::prelude::RawFd {
        return self.sock.into_raw_fd()
    }
}

impl From<UdpSocket> for VDIFUDP {
    fn from(value: UdpSocket) -> Self {
        return Self { sock: value, frame_size: crate::DEFAULT_FRAMESIZE }
    }
}
