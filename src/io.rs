use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::VDIFFrame;

/// A trait indicating a type that can read VDIF frames.
pub trait VDIFRead {
    /// Read a [`VDIFFrame`]
    fn read_frame(&mut self) -> Result<VDIFFrame>;
}

/// A trait indicating a type that can write VDIF frames.
pub trait VDIFWrite {
    /// Write a [`VDIFFrame`].
    fn write_frame(&mut self, frame: &VDIFFrame) -> Result<()>;
}

/// A type capable of reading VDIF frames from any source implementing [`Read`].
pub struct VDIFReader<T: Read> {
    inner: T,
    frame_size: usize,
}

impl<T: Read> VDIFReader<T> {
    /// Construct a new [`VDIFReader`] using `inner` and the specified frame size (total, in bytes).
    pub fn new(inner: T, frame_size: usize) -> Self {
        return Self {
            inner: inner,
            frame_size: frame_size,
        };
    }

    /// Return a reference to the underlying reader.
    pub fn get_ref(&self) -> &T {
        return &self.inner;
    }

    /// Return a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut T {
        return &mut self.inner;
    }

    /// Consume `self` and return the underlying reader.
    pub fn into_inner(self) -> T {
        return self.inner;
    }
}

impl<T: Read> VDIFRead for VDIFReader<T> {
    fn read_frame(&mut self) -> Result<VDIFFrame> {
        // Allocate a frame and read bytes into it
        let mut outframe = VDIFFrame::empty(self.frame_size);
        let bytes_read = self.inner.read(outframe.as_mut_bytes())?;

        if bytes_read == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Reached EOF"));
        } else if bytes_read != self.frame_size {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Did not read a complete VDIF frame",
            ));
        }

        return Ok(outframe);
    }
}

/// A type capable of writing VDIF frames to any destination implementing [`Write`].
pub struct VDIFWriter<T: Write> {
    inner: T,
    frame_size: usize,
}

impl<T: Write> VDIFWriter<T> {
    /// Construct a new [`VDIFWriter`] using `inner` and the specified frame size (total, in bytes).
    pub fn new(inner: T, frame_size: usize) -> Self {
        // Default to a buffer of 10 frames
        return Self {
            inner: inner,
            frame_size: frame_size,
        };
    }

    /// Return a reference to the underlying writer.
    pub fn get_ref(&self) -> &T {
        return &self.inner;
    }

    /// Return a mutable reference to the underlying writer.
    pub fn get_mut(&mut self) -> &mut T {
        return &mut self.inner;
    }

    /// Consume self and return the underlying writer.
    pub fn into_inner(self) -> T {
        return self.inner;
    }
}

impl<T: Write> VDIFWrite for VDIFWriter<T> {
    fn write_frame(&mut self, frame: &VDIFFrame) -> Result<()> {
        assert_eq!(
            self.frame_size,
            frame.bytesize(),
            "VDIF frames must be {} bytes in size for this VDIFWriter",
            self.frame_size
        );
        let _ = self.inner.write(frame.as_bytes())?;
        return Ok(());
    }
}
