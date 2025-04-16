//! Functionality for sending/receiving VDIF data via the User Datagram Protocol (UDP).

use std::net::UdpSocket;
use std::io::Result;

use crate::VDIFFrame;

/// Receive a [`VDIFFrame`] from a [`UdpSocket`]
pub fn recv_frame(sock: &UdpSocket, frame_size: usize) -> Result<VDIFFrame> {
    let mut frame = VDIFFrame::new_empty(frame_size);
    let bytes_read = sock.recv(frame.as_mut_bytes())?;
    if bytes_read != frame_size {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }

    return Ok(frame)
}

/// Send a [`VDIFFrame`] to a [`UdpSocket`]
pub fn send_frame(sock: &UdpSocket, frame: &VDIFFrame) -> Result<()> {
    let _bytes_written = sock.send(frame.as_bytes())?;
    return Ok(())
}