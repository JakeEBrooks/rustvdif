//! Implements the main [`VDIFReader`] and [`VDIFWriter`] types, as well as the [`VDIFRead`] and [`VDIFWrite`] traits.

use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Write, Result};
use std::path::Path;

use crate::VDIFFrame;

/// A trait indicating a type that can read VDIF frames.
pub trait VDIFRead {
    /// Read a [`VDIFFrame`]
    fn read_frame(&mut self) -> Result<VDIFFrame>;
}

/// A trait indicating a type that can write VDIF frames.
pub trait VDIFWrite {
    /// Write a [`VDIFFrame`].
    fn write_frame(&mut self, frame: VDIFFrame) -> Result<()>;
}

/// A type capable of reading VDIF frames from any source implementing [`Read`].
/// 
/// This allows easily reading from VDIF files, for example, like so:
/// 
/// ```rust,ignore
/// fn main() {
///     // A file of 8032 byte VDIF frames
///     let mut file = VDIFReader::open("path/to/my/vdif", 8032).unwrap();
///     // Read the first 100 frames and print header information on each one
///     for _ in 0..100 {
///         let frame = file.read_frame().unwrap();
///         println!("{}", frame.get_header());
///     }
/// }
/// ```
/// 
/// Or from [`TcpStream`](std::net::TcpStream)s:
/// 
/// ```rust,ignore
/// use std::net::TcpStream;
/// 
/// fn main() {
///     // Connect to a TCP stream of VDIF frames
///     let stream = TcpStream::connect("127.0.0.1:34254").unwrap();
///     // VDIFReader is buffered by default, so use a buffer of 100 frames.
///     let mut reader = VDIFReader::with_capacity(stream, 8032, 100);
///     // Read the first 100 frames and print header information on each one
///     for _ in 0..100 {
///         let frame = reader.read_frame().unwrap();
///         println!("{}", frame.get_header());
///     }
/// }
/// ```
/// 
/// [`VDIFReader`]s implement buffered IO by default since VDIF streams are often quite data heavy, so you don't
/// need to worry about using the normal [`BufReader`].
pub struct VDIFReader<T: Read> {
    inner: BufReader<T>,
    frame_size: usize
}

impl<T: Read> VDIFReader<T> {
    /// Construct a new [`VDIFReader`] using `inner` and the specified frame size (total, in bytes).
    pub fn new(inner: T, frame_size: usize) -> Self {
        // Default to a buffer of 10 frames
        return Self { inner: BufReader::with_capacity(10*frame_size, inner), frame_size: frame_size }
    }

    /// Construct a new [`VDIFReader`] using `inner` and the specified frame size and frame capacity. The default 
    /// buffer size is 10 frames.
    pub fn with_capacity(inner: T, frame_size: usize, frame_capacity: usize) -> Self {
        return Self { inner: BufReader::with_capacity(frame_capacity*frame_size, inner), frame_size: frame_size }
    }
}

impl<T: Read> VDIFRead for VDIFReader<T> {
    fn read_frame(&mut self) -> Result<VDIFFrame> {
        // Allocate a frame and read bytes into it
        let mut outframe = VDIFFrame::empty(self.frame_size);
        let bytes_read = self.inner.read(outframe.as_mut_bytes())?;

        if bytes_read == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Reached EOF"))
        } else if bytes_read != self.frame_size {
            return Err(Error::new(ErrorKind::InvalidData, "Did not read a complete VDIF frame"))
        }

        return Ok(outframe);
    }
}

impl VDIFReader<File> {
    /// Open a VDIF file on disk
    pub fn open<P: AsRef<Path>>(path: P, frame_size: usize) -> Result<Self> {
        let file = File::open(path)?;
        // Default to a buffer of 10 frames
        return Ok(Self { inner: BufReader::with_capacity(10*frame_size, file), frame_size: frame_size })
    }

    /// Open a VDIF file on disk with the specified buffer capacity.
    pub fn open_withcapacity<P: AsRef<Path>>(path: P, frame_size: usize, frame_capacity: usize) -> Result<Self> {
        let file = File::open(path)?;
        return Ok(Self { inner: BufReader::with_capacity(frame_capacity*frame_size, file), frame_size: frame_size })
    }
}

/// A type capable of writing VDIF frames to any destination implementing [`Write`].
/// 
/// The behaviour is very similar to [`VDIFReader`].
pub struct VDIFWriter<T: Write> {
    inner: BufWriter<T>,
    frame_size: usize
}

impl<T: Write> VDIFWriter<T> {
    /// Construct a new [`VDIFWriter`] using `inner` and the specified frame size (total, in bytes).
    pub fn new(inner: T, frame_size: usize) -> Self {
        // Default to a buffer of 10 frames
        return Self { inner: BufWriter::with_capacity(10*frame_size, inner), frame_size: frame_size }
    }

    /// Construct a new [`VDIFWriter`] using `inner` and the specified frame size and frame capacity. The default 
    /// buffer size is 10 frames.
    pub fn with_capacity(inner: T, frame_size: usize, frame_capacity: usize) -> Self {
        return Self { inner: BufWriter::with_capacity(frame_capacity*frame_size, inner), frame_size: frame_size }
    }

    /// Flush the contents of the buffer.
    pub fn flush(&mut self) -> Result<()> {
        return self.inner.flush()
    }
}

impl<T: Write> VDIFWrite for VDIFWriter<T> {
    fn write_frame(&mut self, frame: VDIFFrame) -> Result<()> {
        assert_eq!(self.frame_size, frame.bytesize(), "VDIF frames must be {} bytes in size for this VDIFWriter", self.frame_size);
        let _ = self.inner.write(frame.as_bytes())?;
        return Ok(())
    }
}

impl VDIFWriter<File> {
    /// Create a new VDIF file on disk, and attach a [`VDIFWriter`]. The behaviour of this method is similar to
    /// [`create`](std::fs::File::create).
    pub fn create<P: AsRef<Path>>(path: P, frame_size: usize) -> Result<Self> {
        let newfile = File::create(path)?;
        return Ok(Self { inner: BufWriter::with_capacity(10*frame_size, newfile), frame_size: frame_size })
    }

    /// Create a new VDIF file on disk, and attach a [`VDIFWriter`] with the specified buffer size. The behaviour of this method is similar to
    /// [`create`](std::fs::File::create).
    pub fn create_withcapacity<P: AsRef<Path>>(path: P, frame_size: usize, frame_capacity: usize) -> Result<Self> {
        let newfile = File::create(path)?;
        return Ok(Self { inner: BufWriter::with_capacity(frame_capacity*frame_size, newfile), frame_size: frame_size })
    }
}