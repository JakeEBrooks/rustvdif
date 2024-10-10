//! Implements functionality for reading VDIF data from a variety of data sources.

use std::fs::File;
use std::io::{BufReader, Read, Seek, Result};
use std::path::Path;

use crate::frame::VDIFFrame;
use crate::header::VDIFHeader;
use crate::parsing;
use crate::payload::VDIFPayload;

/// Wraps a [`File`] containing VDIF encoded data, and provides methods for extracting
/// that data in a controlled manner.
/// 
/// The methods below behave in a similar manner to [`std::io::Read::read`] in that each read also advances an internal cursor
/// , so be careful of the order that you call these functions in. If possible, constrain your programs to call either
/// [`frame`](VDIFFileReader::frame), [`find_nextframe`](VDIFFileReader::find_nextframe), or sequential blocks of [`header`][`VDIFFileReader::header`] and 
/// [`nextframe`](VDIFFileReader::nextframe).
/// 
/// VDIF files are often quite large, so buffered IO is the default.
pub struct VDIFFileReader {
    file: BufReader<File>,
}


impl VDIFFileReader {
    /// Construct a new [`VDIFFileReader`] from a `BufReader<File>`.
    pub fn new(file: BufReader<File>) -> Self {
        return Self{file: file}
    }

    /// Construct a new [`VDIFFileReader`] from the specified file path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        return Ok(Self{file: BufReader::new(File::open(path)?)})
    }

    /// Construct a new [`VDIFFileReader`] from the specified file path, also specifying the size of the underlying
    /// [`BufReader`].
    pub fn with_capacity<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self> {
        return Ok(Self{file: BufReader::with_capacity(capacity, File::open(path)?)})
    }

    /// Seek to the next frame by advancing the internal cursor by the number of bytes indicated in the
    /// provided [`VDIFHeader`].
    pub fn nextframe(&mut self, header: &VDIFHeader) -> Result<()> {
        let payload_bytesize = header.payload_bytesize();
        self.seek_relative(payload_bytesize as i64)?;
        return Ok(())
    }

    /// Seek to the next frame by first loading the [`VDIFHeader`] of the current frame.
    pub fn find_nextframe(&mut self) -> Result<()> {
        let header = self.header()?;
        self.nextframe(&header)?;
        return Ok(())
    }

    /// Read the currently pointed to [`VDIFHeader`].
    pub fn header(&mut self) -> Result<VDIFHeader> {
        let mut buf: [u8; 32] = [0; 32];
        self.read_exact(&mut buf)?;

        // If this was going to panic, I assume it would have in the above read?
        let (_, header) = parsing::header::parse_header(&buf).unwrap();
        return Ok(header)
    }

    /// Read the currently pointed to payload. Must be called prior to a successful call to [`header`](VDIFFileReader::header).
    pub fn payload(&mut self, header: &VDIFHeader) -> Result<VDIFPayload> {
        let mut buf: Box<[u8]> = vec![0; header.payload_bytesize() as usize].into_boxed_slice();
        self.read_exact(&mut buf)?;

        // If this was going to panic, I assume it would have in the above read?
        let (_, payload) = parsing::payload::parse_payload(&buf, header).unwrap();
        return Ok(payload)
    }

    /// Read the currently pointed to [`VDIFFrame`].
    pub fn frame(&mut self) -> Result<VDIFFrame> {
        let header = self.header()?;
        let payload = self.payload(&header)?;
        return Ok(VDIFFrame::new(header, payload))
    }
}

impl Read for VDIFFileReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        return self.file.read(buf)
    }
}

impl Seek for VDIFFileReader {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        return self.file.seek(pos)
    }
}

// TODO: implement VDIFStreamReader, which is the same as above but accumulates a Vec of VDIFFrames