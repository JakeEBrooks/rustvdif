use std::{mem, net::UdpSocket, os::fd::AsRawFd};

use libc::{c_void, iovec, mmsghdr, recvmmsg, timespec};
use tracing::{event, span, Level};

use crate::VDIFFrame;

pub struct VDIFReceiver {
    sock: UdpSocket,
    _frame_cap: usize,
    frame_num: usize,
    frame_ind: usize,

    msgs: Box<[mmsghdr]>,
    _iovs: Box<[iovec]>,
    bufs: Box<[Box<[u8]>]>,
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

    pub fn recv_batch(&mut self) {
        let span = span!(Level::DEBUG, "recvmmsg");
        let _ = span.enter();

        let res = unsafe { recvmmsg(self.sock.as_raw_fd(), self.msgs.as_mut_ptr(), self.msgs.len() as _, 0, &mut self.timeout) };
        if res < 0 {
            panic!("recvmmsg returned with error code: {}", res);
        }

        self.frame_num = res.try_into().unwrap();
        event!(Level::DEBUG, "Received {res} messages from recvmmsg");
    }

    pub fn recv_frame(&mut self) -> VDIFFrame {
        if self.frame_ind >= self.frame_num {
            self.recv_batch();
            self.frame_ind = 0;
        }

        let outframe = VDIFFrame::from_byte_slice(&self.bufs[self.frame_ind]);
        self.frame_ind += 1;

        return outframe
    }
}