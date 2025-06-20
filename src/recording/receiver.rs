use std::{mem, net::UdpSocket, os::fd::AsRawFd};

use libc::{c_void, iovec, mmsghdr, recvmmsg, timespec};
use tracing::{event, Level};

use crate::VDIFFrame;

/// A high performance UDP packet receiver designed to handle large input data rates. Internally uses the [`recvmmsg`] system call
/// to reduce the overhead of going through the OS
pub struct VDIFReceiver {
    sock: UdpSocket,
    _frame_cap: usize,
    frame_num: usize,
    frame_ind: usize,

    msgs: Box<[mmsghdr]>,
    _iovs: Box<[iovec]>,
    pub(crate) bufs: Box<[Box<[u8]>]>,
    timeout: timespec
}

impl VDIFReceiver {
    pub fn new(socket: UdpSocket, packet_size: usize, framebuf_size: usize) -> Self {
        let vlen = framebuf_size;
        let mut msgs: Box<[mmsghdr]> = unsafe { vec![mem::zeroed(); vlen].into_boxed_slice() };
        let mut iovs: Box<[iovec]> = unsafe { vec![mem::zeroed(); vlen].into_boxed_slice() };
        let mut bufs: Box<[Box<[u8]>]> = vec![vec![0u8; packet_size].into_boxed_slice(); vlen].into_boxed_slice();
        for i in 0..vlen {
            iovs[i].iov_base = bufs[i].as_mut_ptr() as *mut c_void;
            iovs[i].iov_len = packet_size;
            msgs[i].msg_hdr.msg_iov = &mut iovs[i];
            msgs[i].msg_hdr.msg_iovlen = 1;
        };

        let timeout = timespec { tv_sec: 1, tv_nsec: 0 };

        return Self { sock: socket, _frame_cap: vlen, frame_num: 0, frame_ind: 0, msgs: msgs, _iovs: iovs, bufs: bufs, timeout: timeout }
    }

    fn recv_batch(&mut self) {
        let res = unsafe { recvmmsg(self.sock.as_raw_fd(), self.msgs.as_mut_ptr(), self.msgs.len() as _, 0, &mut self.timeout) };
        if res < 0 {
            panic!("recvmmsg returned with error code: {}", res);
        }
        event!(Level::DEBUG, "Received {res} frames from recvmmsg");
        self.frame_num = res.try_into().unwrap();
    }

    pub fn recv_frame(&mut self) -> VDIFFrame {
        if self.frame_ind >= self.frame_num {
            self.recv_batch();
            self.frame_ind = 0;
        };

        let outframe = VDIFFrame::from_byte_slice(&self.bufs[self.frame_ind]);
        self.frame_ind += 1;

        return outframe
    }

    pub fn recv_vtp_frame(&mut self) -> (u64, VDIFFrame) {
        if self.frame_ind >= self.frame_num {
            self.recv_batch();
            self.frame_ind = 0;
        };

        let buf: &[u8] = &self.bufs[self.frame_ind];
        let seq = u64::from_le_bytes(buf[0..8].try_into().unwrap());
        let frame = VDIFFrame::from_byte_slice(&buf[8..]);
        self.frame_ind += 1;

        return (seq, frame)
    }
}