use std::{io::{Error, Result}, mem, net::UdpSocket, os::fd::AsRawFd};

use libc::{c_void, iovec, mmsghdr, recvmmsg, timespec};

use crate::VDIFFrame;

/// A high performance VDIF/UDP packet receiver designed to handle large input data rates.
/// 
/// Internally uses the [`recvmmsg`] system call to reduce the overhead of going through the OS.
pub struct UDPSocketBuf {
    sock: UdpSocket,
    frame_cap: usize,
    frame_num: usize,
    frame_ind: usize,
    /// Counts the number of packets received so far
    pub packet_count: u64,

    msgs: Box<[mmsghdr]>,
    _iovs: Box<[iovec]>,
    bufs: Box<[Box<[u32]>]>,
    timeout: timespec
}

impl UDPSocketBuf {
    /// Create a new socket buffer attached to `socket`.
    /// 
    /// The internal buffer can hold a total of `framebuf_size` frames of size `frame_size` at any point.
    pub fn new(socket: UdpSocket, frame_size: usize, framebuf_size: usize) -> Self {
        let vlen = framebuf_size;
        let mut msgs: Box<[mmsghdr]> = unsafe { vec![mem::zeroed(); vlen].into_boxed_slice() };
        let mut _iovs: Box<[iovec]> = unsafe { vec![mem::zeroed(); vlen].into_boxed_slice() };
        let mut bufs: Box<[Box<[u32]>]> = vec![vec![0u32; frame_size/4].into_boxed_slice(); vlen].into_boxed_slice();
        for i in 0..vlen {
            _iovs[i].iov_base = bufs[i].as_mut_ptr() as *mut c_void;
            _iovs[i].iov_len = frame_size;
            msgs[i].msg_hdr.msg_iov = &mut _iovs[i];
            msgs[i].msg_hdr.msg_iovlen = 1;
        };

        let timeout = timespec { tv_sec: 1, tv_nsec: 0 };

        return Self { sock: socket, frame_cap: vlen, frame_num: 0, frame_ind: 0, packet_count: 0, msgs, _iovs, bufs, timeout }
    }

    /// Attempt to fill the internal buffer with packets from the socket by calling [`recvmmsg`].
    /// 
    /// This will overwrite the contents of the buffer, so ensure that you have fetched all the data you need before calling this.
    pub fn recv_batch(&mut self) -> Result<usize> {
        let res = unsafe { recvmmsg(self.sock.as_raw_fd(), self.msgs.as_mut_ptr(), self.msgs.len() as _, 0, &mut self.timeout) };
        if res < 0 {
            return Err(Error::last_os_error());
        };
        debug_assert!(res <= self.frame_cap as i32);
        self.packet_count += self.frame_num as u64;
        return Ok(res as usize)
    }

    /// Receive a [`VDIFFrame`] from the internal buffer.
    /// 
    /// If all frames have been received, this function will automatically call [`recv_batch`](Self::recv_batch) to retrieve more data. Therefore, the user
    /// shouldn't need to ever worry about calling [`recv_batch`](Self::recv_batch).
    pub fn recv_frame(&mut self) -> Result<VDIFFrame> {
        let mut outframe = VDIFFrame::new_empty(self.bufs[0].len()*4);
        self.recv_frame_to(outframe.as_mut_slice())?;
        return Ok(outframe)
    }

    /// Receive a single frame from the internal buffer directed to `dest`.
    /// 
    /// If all frames have been received, this function will automatically call [`recv_batch`](Self::recv_batch) to retrieve more data. Therefore, the user
    /// shouldn't need to ever worry about calling [`recv_batch`](Self::recv_batch).
    pub fn recv_frame_to(&mut self, dest: &mut [u32]) -> Result<()> {
        if self.frame_ind >= self.frame_num {
            self.frame_num = self.recv_batch()?;
            self.frame_ind = 0;
        };

        dest.copy_from_slice(&self.bufs[self.frame_ind]);
        self.frame_ind += 1;
        return Ok(())
    }
}

unsafe impl Send for UDPSocketBuf {}
unsafe impl Sync for UDPSocketBuf {}