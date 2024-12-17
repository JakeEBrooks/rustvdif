//! Implements functionality for generating a stream of VDIF frames for testing purposes.

use crate::{header::VDIFHeader, header_encoding::encode_header, io::VDIFRead, VDIFFrame};

/// Allows the generation of test VDIF frames.
pub struct VDIFSim {
    frame_size: u32,
    frame_rate: usize,
    thread_no: usize,

    current_frame: u32,
    current_thread: u16,
    current_time: u32,
}

impl VDIFSim {
    /// Construct a new [`VDIFSim`].
    ///
    /// `frame_rate` is the the number of frames contained within one second *per* thread.
    pub fn new(frame_size: usize, frame_rate: usize, thread_no: usize) -> Self {
        return Self {
            frame_size: frame_size as u32,
            frame_rate: frame_rate,
            thread_no: thread_no,
            current_frame: 0,
            current_thread: 0,
            current_time: 0,
        };
    }

    /// Generate a [`VDIFFrame`].
    ///
    /// The generated VDIF frame contains the following header fields:
    ///
    /// `
    /// is_valid: true,
    /// is_legacy: false,
    /// time: [current_time],
    /// epoch: 3,
    /// frameno: [current_frame],
    /// version: 0,
    /// channels: 0,
    /// size: frame_size/8,
    /// is_real: true,
    /// bits_per_sample: 2,
    /// thread: [current_thread],
    /// station: 134,
    /// edv0: 0,
    /// edv1: 0,
    /// edv2: 0,
    /// edv3: 0
    /// `
    ///
    /// All data samples are set to zero, and `current_` variables are incremented properly when this function is called.
    /// The internal counters are incremented in the following order: [current_frame] -> [current_thread] -> [current_time].
    /// The generated VDIF frames are only valid for six months since the `epoch` field is not
    /// handled; you wouldn't generate six months worth of data, would you?
    pub fn generate_frame(&mut self) -> VDIFFrame {
        let mut out = VDIFFrame::empty(self.frame_size as usize);
        let outheader = VDIFHeader {
            is_valid: true,
            is_legacy: false,
            time: self.current_time,
            epoch: 3,
            frameno: self.current_frame,
            version: 0,
            channels: 0,
            size: self.frame_size / 8,
            is_real: true,
            bits_per_sample: 2,
            thread: self.current_thread,
            station: 134,
            edv0: 0,
            edv1: 0,
            edv2: 0,
            edv3: 0,
        };

        let encoded_header = encode_header(outheader);
        for i in 0..8 {
            out.as_mut_slice()[i] = encoded_header[i];
        }

        if self.current_frame >= (self.frame_rate as u32) - 1 {
            self.current_frame = 0;
            if self.current_thread == (self.thread_no - 1) as u16 {
                self.current_thread = 0;
                self.current_time += 1;
            } else {
                self.current_thread += 1;
            }
        } else {
            self.current_frame += 1;
        }

        return out;
    }
}

impl VDIFRead for VDIFSim {
    fn read_frame(&mut self) -> std::io::Result<VDIFFrame> {
        return Ok(self.generate_frame());
    }
}
