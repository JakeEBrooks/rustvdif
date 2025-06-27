//! Functionality for sending/receiving VDIF data via the VDIF Transport Protocol (VTP).
//! 
//! A VDIF frame sent via VTP is simply labelled with a 64-bit sequence number placed before the header. It is formally defined
//! in the [VTP specification](https://vlbi.org/vlbi-standards/).

use std::mem::{transmute, MaybeUninit};
use std::net::UdpSocket;
use std::io::Result;

use crate::VDIFFrame;

/// Receive a [`VDIFFrame`] from a [`UdpSocket`], along with its VTP sequence number
pub fn recv_vtp_frame(sock: &UdpSocket, frame_size: usize) -> Result<(u64, VDIFFrame)> {
    let buf: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(frame_size + 8);
    let bytes_read = sock.recv(unsafe { std::slice::from_raw_parts_mut(buf.as_ptr() as *mut u8, buf.len()) })?;

    if bytes_read != frame_size + 8 {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }

    let buf_init = unsafe { buf.assume_init() };
    let seq = u64::from_le_bytes(buf_init[0..8].try_into().unwrap());
    let frame = VDIFFrame::from_byte_slice(&buf_init[8..frame_size+8]);
    return Ok((seq, frame))
}

/// Send a [`VDIFFrame`] to a [`UdpSocket`], along with a VTP sequence number
pub fn send_vtp_frame(sock: &UdpSocket, seq: u64, frame: &VDIFFrame) -> Result<()> {
    let mut sendbuf: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(frame.bytesize() + 8);
    sendbuf[0..8].copy_from_slice(unsafe { transmute::<&[u8], &[std::mem::MaybeUninit<u8>]>(seq.to_le_bytes().as_slice()) });
    sendbuf[8..frame.bytesize() + 8].copy_from_slice(unsafe { transmute::<&[u8], &[std::mem::MaybeUninit<u8>]>(frame.as_bytes()) });
    let sendbuf_init = unsafe { sendbuf.assume_init() };
    sock.send(&sendbuf_init)?;
    return Ok(())
}
