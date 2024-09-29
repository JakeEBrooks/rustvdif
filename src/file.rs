//! Implements [`File`] based operations using VDIF data.

use std::fs::File;
use std::io::{BufReader, BufWriter, Result, Write};
use std::path::Path;

use crate::read::VDIFReader;
use crate::VDIFDataFrame;

/// Create a [`VDIFReader<File>`] from the specified file path.
/// 
/// *Consider using [`fopen_buf`] for buffered IO instead.*
pub fn fopen<P: AsRef<Path>>(path: P) -> Result<VDIFReader<File>> {
    let file = File::open(path)?;
    return Ok(VDIFReader::new(file)?)
}

/// Create a [`VDIFReader<BufReader<File>>`] from the specified file path.
/// 
/// If you want to enlarge the default [`BufReader`] capacity, create a [`VDIFReader`] manually
/// using [`VDIFReader::new`], and pass in a [`BufReader::with_capacity`].
pub fn fopen_buf<P: AsRef<Path>>(path: P) -> Result<VDIFReader<BufReader<File>>> {
    let file = BufReader::new(File::open(path)?);
    return Ok(VDIFReader::new(file)?)
}

/// Write a [`Vec`] of [`VDIFDataFrame`]s to a file on disk.
pub fn writeto<P: AsRef<Path>>(path: P, data: &Vec<VDIFDataFrame>) -> Result<()> {
    let mut file = BufWriter::new(File::open(path)?);
    for frame in data {
        _ = file.write(frame.get_header_data())?;
        _ = file.write(frame.get_payload())?;
    }
    file.flush()?;
    return Ok(())
}