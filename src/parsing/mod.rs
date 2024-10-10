//! Provides functionality for parsing streams of bytes into various VDIF objects.
//! 
//! The parsing is done using [`nom`] and all the parsing functions are [`nom`] parsers,
//! so see their documentation to understand these functions.

pub mod header;

pub mod payload {
    //! Implements [`parse_payload`].
    use nom::IResult;
    use nom::number::complete::le_u32;

    use crate::header::VDIFHeader;
    use crate::payload::VDIFPayload;

    /// Parse a [`VDIFPayload`] from a byte slice. Requires a reference to the associated [`VDIFHeader`] 
    /// to ensure the correct number of bytes are parsed.
    pub fn parse_payload<'a, 'b>(input: &'a [u8], header: &'b VDIFHeader) -> IResult<&'a [u8], VDIFPayload> {
        let payload_wordsize = header.payload_wordsize();
        let mut out: Vec<u32> = Vec::with_capacity(payload_wordsize as usize);
        let (mut remaining, mut word) = le_u32(input)?;
        out.push(word);
        for _ in 1..payload_wordsize {
            (remaining, word) = le_u32(remaining)?;
            out.push(word);
        }

        return Ok((remaining, VDIFPayload::new(out.into_boxed_slice())))
    }
}

pub mod frame {
    //! Implements [`parse_frame`].
    use nom::IResult;

    use crate::header::VDIFHeader;
    use crate::payload::VDIFPayload;
    use crate::parsing::{header::parse_header, payload::parse_payload};

    /// Parse a [`VDIFHeader`] and the associated [`VDIFPayload`] from a byte slice.
    pub fn parse_frame(input: &[u8]) -> IResult<&[u8], (VDIFHeader, VDIFPayload)> {
        let (remaining, header) = parse_header(input)?;
        let (remaining, payload) = parse_payload(remaining, &header)?;
        return Ok((remaining, (header, payload)))
    }
}