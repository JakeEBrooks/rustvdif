//! Extends [`std`] functionality for VDIF data, and implements the central [`VDIFReader`] object of this library.

use std::io::{Read, Result, Seek, SeekFrom};

use crate::{header::{VDIFHeader, VDIF_HEADER_SIZE}, VDIFDataFrame};

/// Extends the base [`Read`] trait with convenience functions for reading VDIF data.
/// 
/// Note that these methods do not perform any form of seeking, and behave similarly
/// to [std::io::Read::read].
pub trait VDIFRead: Read {
    /// Create a [`VDIFHeader`] by reading exactly the number of bytes required.
    fn read_vdif_header(&mut self) -> Result<VDIFHeader> {
        let mut buf: [u8; VDIF_HEADER_SIZE] = [0; VDIF_HEADER_SIZE];
        self.read_exact(&mut buf)?;
        return Ok(VDIFHeader::new(buf))
    }

    /// Create a byte slice representing a VDIF payload by reading exactly 
    /// the number of payload bytes indicated in the provided header.
    fn read_vdif_payload(&mut self, header: &VDIFHeader) -> Result<Box<[u8]>> {
        let payload_size: u32 = header.byte_size() - VDIF_HEADER_SIZE as u32;
        let mut buf: Box<[u8]> = vec![0; payload_size as usize].into_boxed_slice();
        self.read_exact(&mut buf)?;

        return Ok(buf)
    }

    /// Create a [`VDIFDataFrame`] by first reading header information, and then reading the associated payload.
    fn read_vdif_frame(&mut self) -> Result<VDIFDataFrame> {
        let header = self.read_vdif_header()?;
        let payload = self.read_vdif_payload(&header)?;
        return Ok(VDIFDataFrame { header: header, payload: payload })
    }
}

/// Extend all other types implementing [`Read`] with [`VDIFRead`]. This may be removed as it
/// not an opt-in feature.
impl<T: Read> VDIFRead for T {}

/// The primary way of interacting with VDIF data.
/// 
/// This object wraps some other object implementing [`VDIFRead`] (essentially just [`std::io::Read`]) and uses a
/// cursor to track the currently hovered VDIF data frame. VDIF data streams can be moved through using 
/// [`next`](VDIFReader::next) and [`find_next`](VDIFReader::find_next). The currently hovered data frame can be
/// accessed in different ways through a [`VDIFReader`]s interface.
/// 
/// # Examples
/// Any type that implements [`Read`] and [`Seek`] can be wrapped in a [`VDIFReader`], including [`std::fs::File`]:
/// ```rust
/// let file = File::open("my/vdif/file").unwrap();
/// let mut reader = VDIFReader::new(file).unwrap();
/// println!("{}", reader.get_header().unwrap());
/// ```
/// When interacting with anything performing many system calls (like [`File`](std::fs::File)s or 
/// [`TcpStream`](std::net::TcpStream)s), you should consider wrapping them in a [`BufReader`](std::io::BufReader):
/// ```rust
/// let file = BufReader::new(File::open("my/vdif/file").unwrap());
/// let mut reader = VDIFReader::new(file).unwrap();
/// // Print the first 10 headers.
/// for i in 0..10 {
///     let header = reader.get_header().unwrap();
///     println!("{}", header);
///     reader.next(&header).unwrap();
/// };
/// ```
/// *Note that you should try to avoid using [`unwrap`](std::result::Result::unwrap) in your applications.*
pub struct VDIFReader<T: VDIFRead + Seek> {
    reader: T,
    // Always points to the starting byte of the current DataFrame
    cursor: u64
}

impl<T: VDIFRead + Seek> VDIFReader<T> {
    /// Construct a new [`VDIFReader`] from a type implementing [`Read`] and [`Seek`]. 
    /// Performs a [`rewind`](std::io::Seek::rewind) before taking ownership.
    pub fn new(mut reader: T) -> Result<Self> {
        reader.rewind()?;
        return Ok(Self{reader: reader, cursor: 0})
    }

    /// Get the [`VDIFHeader`] of the currently pointed to data frame.
    pub fn get_header(&mut self) -> Result<VDIFHeader> {
        self.reader.seek(SeekFrom::Start(self.cursor))?;
        return self.reader.read_vdif_header()
    }

    /// Get the payload of the currently pointed to data frame by first calling [`get_header`](VDIFReader::get_header).
    pub fn get_payload(&mut self) -> Result<Box<[u8]>> {
        self.reader.seek(SeekFrom::Start(self.cursor))?;
        let header = self.get_header()?;
        return self.reader.read_vdif_payload(&header)
    }

    /// Get the payload associated with the currently pointed to data frame by passing the associated [`VDIFHeader`].
    /// Useful for when you have already read the header information and don't want to perform an unnecessary copy.
    pub fn get_attachedpayload(&mut self, header: &VDIFHeader) -> Result<Box<[u8]>> {
        self.reader.seek(SeekFrom::Start(self.cursor + VDIF_HEADER_SIZE as u64))?;
        return self.reader.read_vdif_payload(header)
    }

    /// Get the currently pointed to [`VDIFDataFrame`].
    pub fn get_frame(&mut self) -> Result<VDIFDataFrame> {
        self.reader.seek(SeekFrom::Start(self.cursor))?;
        let frame = self.reader.read_vdif_frame()?;
        return Ok(frame)
    }

    /// Seek to the next data frame using the currently pointed to [`VDIFHeader`].
    /// Useful for when you have already read the header information and don't want to perform an unnecessary copy.
    pub fn next(&mut self, header: &VDIFHeader) -> Result<()> {
        let frame_size = header.byte_size();
        self.cursor += frame_size as u64;
        self.reader.seek(SeekFrom::Start(self.cursor))?;
        return Ok(())
    }

    /// Seek to the next data frame by first reading the currently pointed to [`VDIFHeader`].
    pub fn find_next(&mut self) -> Result<()> {
        let current_header = self.get_header()?;
        let frame_size = current_header.byte_size();
        self.cursor += frame_size as u64;
        self.reader.seek(SeekFrom::Start(self.cursor))?;
        return Ok(())
    }
}