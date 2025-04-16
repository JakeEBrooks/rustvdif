use std::io::{Read, Result, Write};

use crate::VDIFFrame;

/// Read a VDIF frame from any [`Read`] type
pub fn read_frame<T: Read>(reader: &mut T, frame_size: usize) -> Result<VDIFFrame> {
    // Allocate but don't initialise the heap memory for the output frame
    let mut buf: Box<[std::mem::MaybeUninit<u32>]> = Box::new_uninit_slice(frame_size / 4);
    // Read bytes into the frame memory
    let bytes_read = reader.read(
        unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, frame_size) }
    )?;

    // If we didn't get exactly one frame, return EOF
    if bytes_read != frame_size {
        return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
    }

    return Ok(VDIFFrame::new(unsafe { buf.assume_init() }))
}

/// Read a VDIF frame from any [`Read`] type, along with its VTP sequence number
pub fn read_vtp_frame<T: Read>(reader: &mut T, frame_size: usize) -> Result<(u64, VDIFFrame)> {
    let mut seqbuf: [u8; 8] = [0; 8];
    let seq_bytes_read = reader.read(&mut seqbuf)?;
    assert_eq!(seq_bytes_read, 8, "Did not read a full VTP sequence number");

    return Ok((u64::from_le_bytes(seqbuf), read_frame(reader, frame_size)?))
}

/// Write a VDIF frame to any [`Write`] type
pub fn write_frame<T: Write>(writer: &mut T, frame: VDIFFrame) -> Result<()> {
    let _bytes_written = writer.write(frame.as_bytes())?;
    return Ok(())
}

/// Write a VDIF frame to any [`Write`] type, along with a `u64` VTP sequence number
pub fn write_vtp_frame<T: Write>(writer: &mut T, seq: u64, frame: VDIFFrame) -> Result<()> {
    let _bytes_written = writer.write(&seq.to_le_bytes())?;
    return write_frame(writer, frame)
}