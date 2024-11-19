//! A simple implementation of VDIF over UDP.
//! 
//! Doesn't perform any buffering or packet reordering, so frames may not be read in order.
//! Assumes that a single, complete VDIF frame is fully contained within a single datagram.

use std::io::Result;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use crate::io::{VDIFRead, VDIFWrite};
use crate::VDIFFrame;

/// The type for reading VDIF frames from a UDP socket.
pub struct VDIFUDP {
    /// The inner [`UdpSocket`]
    pub sock: UdpSocket,
    frame_size: usize
}

impl VDIFUDP {
    /// Construct a new [`VDIFUDP`] type attached to a specific [`SocketAddr`].
    pub fn new<A: ToSocketAddrs>(addr: A, frame_size: usize) -> Result<Self> {
        let sock = UdpSocket::bind(addr)?;
        return Ok(Self { sock: sock, frame_size: frame_size })
    }

    /// [`recv`](std::net::UdpSocket::recv) a [`VDIFFrame`].
    pub fn recv_frame(&mut self) -> Result<VDIFFrame> {
        let mut frame = VDIFFrame::empty(self.frame_size);
        self.sock.recv(frame.as_bytes_mut())?;
        return Ok(frame)
    }

    /// [`recv_from`](std::net::UdpSocket::recv_from) a [`VDIFFrame`], additionally returning the source address. 
    pub fn recv_frame_from(&mut self) -> Result<(VDIFFrame, SocketAddr)> {
        let mut frame = VDIFFrame::empty(self.frame_size);
        let (_, src) = self.sock.recv_from(frame.as_bytes_mut())?;
        return Ok((frame, src))
    }

    /// [`send`](std::net::UdpSocket::send) a [`VDIFFrame`] to the attached address.
    pub fn send_frame(&self, frame: VDIFFrame) -> Result<()> {
        let _ = self.sock.send(frame.as_bytes())?;
        return Ok(())
    }

    /// [`send_to`](std::net::UdpSocket::send_to) a [`VDIFFrame`] to the *specified* address.
    pub fn send_frame_to<A: ToSocketAddrs>(&self, addr: A, frame: VDIFFrame) -> Result<()> {
        let _ = self.sock.send_to(frame.as_bytes(), addr)?;
        return Ok(())
    }
}

impl VDIFRead for VDIFUDP {
    fn read_frame(&mut self) -> Result<VDIFFrame> {
        return self.recv_frame()
    }
}

impl VDIFWrite for VDIFUDP {
    fn write_frame(&mut self, frame: VDIFFrame) -> Result<()> {
        return self.send_frame(frame)
    }
}