//! Implements the central [`VDIFReader`] object of this library.

use std::{io::{BufReader, ErrorKind, Read, Result}, path::Path};
use std::fs::File;

use crate::{frame::VDIFFrame, header::{VDIFHeader, VDIF_HEADER_BYTESIZE}, payload::VDIFPayload};

pub struct VDIFReader<T: Read> {
    reader: T
}

impl<T: Read> VDIFReader<T> {
    pub fn new(reader: T) -> Self {
        return Self{reader: reader}
    }

    pub fn get_header(&mut self) -> Result<VDIFHeader> {
        let mut buf: [u8; VDIF_HEADER_BYTESIZE] = [0; VDIF_HEADER_BYTESIZE];
        self.read_exact(&mut buf)?;
        return Ok(VDIFHeader::frombytes(buf))
    }

    pub fn get_payload(&mut self, header: &VDIFHeader) -> Result<VDIFPayload> {
        let mut buf: Box<[u8]> = vec![0; header.payload_bytesize() as usize].into_boxed_slice();
        self.read_exact(&mut buf)?;
        return VDIFPayload::frombytes(buf, header)
    }

    pub fn get_frame(&mut self) -> Result<VDIFFrame> {
        let header = self.get_header()?;
        let payload = self.get_payload(&header)?;
        return Ok(VDIFFrame::new(header, payload))
    }

    pub fn get_frame_set(&mut self) -> Result<Vec<VDIFFrame>> {
        let mut cont: bool = true;
        let mut out: Vec<VDIFFrame> = Vec::new();
        let mut frame_result = self.get_frame();
        let time_start: u32;

        // Grab an initial frame to know what time segment we're looking for
        match frame_result {
            Ok(frame) => {
                time_start = frame.get_header().raw_time();
                out.push(frame);
            },
            Err(e) => return Err(e)
        }

        // As long as we're looking at the same time segment, and we're not at EOF, keep adding
        // frames to the output
        while cont {
            frame_result = self.get_frame();
            match frame_result {
                Ok(frame) => {
                    if frame.get_header().raw_time() != time_start {
                        cont = false
                    } else {
                        out.push(frame);
                    }
                },
                Err(error) => match error.kind() {
                    ErrorKind::UnexpectedEof => cont = false,
                    _other => return Err(error)
                }
            }
        }
        return Ok(out)
    }
}

impl<T: Read> std::io::Read for VDIFReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        return self.reader.read(buf)
    }
}

impl VDIFReader<BufReader<File>> {
    pub fn fopen<P: AsRef<Path>>(path: P) -> Result<Self> {
        return Ok(Self::new(BufReader::new(File::open(path)?)))
    }

    pub fn fopen_with_capacity<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self> {
        return Ok(Self::new(BufReader::with_capacity(capacity, File::open(path)?)))
    }
}