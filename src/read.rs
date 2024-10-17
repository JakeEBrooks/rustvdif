//! Provides functionality for reading VDIF data from a variety of data sources.

use std::fs::File;
use std::io::{BufReader, Read, Result, Seek};
use std::path::Path;

use crate::frame::VDIFFrame;
use crate::header::VDIFHeader;
use crate::parsing::{
    parse_all_frames, parse_frame, parse_header, parse_payload, VDIF_HEADER_BYTESIZE,
};
use crate::payload::VDIFPayload;

/// A wrapper around any type implementing [`Read`], with added methods for reading
/// VDIF types directly.
///
/// The [`VDIFReader`] methods behave in a similar manner to [`std::io::Read::read`] in that each `get_`
/// method also advances an internal cursor, so be careful of the order that you call these functions in.
pub struct VDIFReader<Inner: Read> {
    inner: Inner,
}

impl<T: Read> VDIFReader<T> {
    /// Construct a new [`VDIFReader`] from the given type implementing [`Read`].
    pub fn new(inner: T) -> Self {
        return Self { inner: inner };
    }

    /// Get a [`VDIFHeader`].
    pub fn get_header(&mut self) -> Result<VDIFHeader> {
        let mut buf: [u8; VDIF_HEADER_BYTESIZE] = [0; VDIF_HEADER_BYTESIZE];
        self.inner.read_exact(&mut buf)?;
        let (_, outframe) = parse_header(&buf).unwrap();
        return Ok(outframe);
    }

    /// Get a [`VDIFPayload`]. This requires passing in the associated header to determine how many bytes to read.
    pub fn get_payload(&mut self, header: &VDIFHeader) -> Result<VDIFPayload> {
        let mut buf: Box<[u8]> = vec![0; header.payload_bytesize() as usize].into_boxed_slice();
        self.inner.read_exact(&mut buf)?;
        let (_, outpayload) = parse_payload(&buf, header).unwrap();
        return Ok(outpayload);
    }

    /// Get a [`VDIFFrame`].
    pub fn get_frame(&mut self) -> Result<VDIFFrame> {
        let header = self.get_header()?;
        let payload = self.get_payload(&header)?;
        return Ok(VDIFFrame::new(header, payload));
    }

    /// Get a [`VDIFFrame`] unpacked into a [`VDIFHeader`] and [`VDIFPayload`].
    pub fn get_frame_unpacked(&mut self) -> Result<(VDIFHeader, VDIFPayload)> {
        let header = self.get_header()?;
        let payload = self.get_payload(&header)?;
        return Ok((header, payload));
    }

    /// The [`get_frame`](VDIFReader::get_frame) method does not know the size of the frame in advance, and therefore performs
    /// two read operations to get the header, determine the size of the payload from that, and then get the payload.
    /// However, if you can **guarantee** the byte size of each frame in advance, use this method instead to read a
    /// full frame in a single operation.
    pub fn get_sized_frame(&mut self, framesize: usize) -> Result<VDIFFrame> {
        let mut buf: Box<[u8]> = vec![0; framesize].into_boxed_slice();
        self.inner.read_exact(&mut buf)?;
        let (_, frame) = parse_frame(&buf).unwrap();
        return Ok(frame);
    }
}

/// WARNING: This is included for full functionality, but its use is discouraged since you can mess
/// up the inetrnal cursor by not reading an exact integer number of VDIF frames. Use this only when you know
/// the exact layout of your VDIF data in advance.
impl<T: Read> std::io::Read for VDIFReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        return self.inner.read(buf);
    }
}

/// Wraps a [`File`] containing VDIF encoded data.
///
/// VDIF files are often quite large, so buffered IO is the default.
pub struct VDIFFileReader {
    reader: VDIFReader<BufReader<File>>,
}

impl VDIFFileReader {
    /// Construct a new [`VDIFFileReader`] from a [`File`].
    pub fn new(file: File) -> Self {
        return Self {
            reader: VDIFReader::new(BufReader::new(file)),
        };
    }

    /// Construct a new [`VDIFFileReader`] from the specified file path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        return Ok(Self {
            reader: VDIFReader::new(BufReader::new(File::open(path)?)),
        });
    }

    /// Construct a new [`VDIFFileReader`] from the specified file path, also specifying the size of the underlying
    /// [`BufReader`].
    pub fn with_capacity<P: AsRef<Path>>(path: P, capacity: usize) -> Result<Self> {
        return Ok(Self {
            reader: VDIFReader::new(BufReader::with_capacity(capacity, File::open(path)?)),
        });
    }

    /// Read the currently pointed to [`VDIFFrame`].
    pub fn get_frame(&mut self) -> Result<VDIFFrame> {
        return self.reader.get_frame();
    }

    /// Read the currently pointed to [`VDIFFrame`].
    pub fn get_frame_unpacked(&mut self) -> Result<(VDIFHeader, VDIFPayload)> {
        return self.reader.get_frame_unpacked();
    }

    /// See [`VDIFReader::get_sized_frame`].
    pub fn get_sized_frame(&mut self, framesize: usize) -> Result<VDIFFrame> {
        return self.reader.get_sized_frame(framesize);
    }

    /// Read all VDIF frames from the file.
    ///
    /// WARNING: This could be an expensive operation depending on the file size.
    pub fn get_all_frames(&mut self) -> Result<Vec<VDIFFrame>> {
        let mut bytes: Vec<u8> = Vec::new();
        self.reader.read_to_end(&mut bytes)?;
        let (_, frames) = parse_all_frames(&bytes).unwrap();
        return Ok(frames);
    }

    /// Moves the inernal file cursor to the next frame without processing the payload.
    pub fn nextframe(&mut self) -> Result<()> {
        let header = self.reader.get_header()?;
        self.reader
            .inner
            .seek_relative(header.payload_bytesize() as i64)?;
        return Ok(());
    }
}

/// WARNING: This is included for full functionality, but its use is discouraged since you can mess
/// up the file cursor by not reading an exact integer number of VDIF frames. Use this only when you know
/// the exact layout of your VDIF file in advance.
impl Read for VDIFFileReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        return self.reader.read(buf);
    }
}

/// WARNING: This is included for full functionality, but its use is discouraged since you can mess
/// up the file cursor by not seeking an exact integer number of VDIF frames.
/// Try to use [`nextframe`](VDIFFileReader::nextframe) instead, unless you know the exact layout of your
/// file in advance.
impl Seek for VDIFFileReader {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        return self.reader.inner.seek(pos);
    }
}
