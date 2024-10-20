//! Provides functionality for interacting with VDIF headers and header information.

use time::{Date, PrimitiveDateTime, Time};

use crate::{encoding::encode_header, parsing::{VDIF_HEADER_BYTESIZE, VDIF_HEADER_SIZE}};

/// Station identifiers can be either a two character ASCII string, or a numeric ID.
pub enum StationID {
    /// The station ID as a string
    StringID(String),
    /// The station ID as a number
    NumericID(u16),
}

/// A VDIF data frame header containing all the information defined in the VDIF specification.
///
/// The header information is accessed through public fields, and through various methods. Note that
/// functions and methods returning various sizes refer to the encoded VDIF data frame,
/// not the decoded [`VDIFFrame`](super::frame::VDIFFrame) type.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct VDIFHeader {
    /// Whether the frame is valid.
    pub is_valid: bool,
    /// Whether the frame is a legacy VDIF data frame.
    pub is_legacy: bool,
    /// The raw timestamp of the frame
    pub time: u32,
    /// The raw reference epoch of the frame.
    pub epoch: u8,
    /// The frame number.
    pub frame: u32,
    /// The VDIF version.
    pub version: u8,
    /// The number of channels within the frame.
    pub channels: u8,
    /// The size in bytes of the data frame (header **and** payload).
    pub size: u32,
    /// Whether the encoded data is real or complex.
    pub is_real: bool,
    /// The bits/sample of the encoded data.
    pub bits_per_sample: u8,
    /// The thread ID of the frame.
    pub thread: u16,
    /// The source station of the frame.
    pub station: u16,

    /// EDV word 0.
    pub edv0: u32,
    /// EDV word 1.
    pub edv1: u32,
    /// EDV word 2.
    pub edv2: u32,
    /// EDV word 3.
    pub edv3: u32,
}

impl VDIFHeader {
    /// Return the size in bytes of the associated data frame.
    pub fn bytesize(&self) -> u32 {
        return self.size;
    }

    /// Return the size in bytes of the payload associated with this header
    pub fn payload_bytesize(&self) -> u32 {
        return self.size - (VDIF_HEADER_BYTESIZE as u32);
    }

    /// Return the size in 32-bit words of the associated data frame
    pub fn wordsize(&self) -> u32 {
        return self.size / 4;
    }

    /// Return the size in 32-bit words of the payload associated with this header.
    pub fn payload_wordsize(&self) -> u32 {
        return self.wordsize() - (VDIF_HEADER_SIZE as u32);
    }

    /// Get the timestamp of this header as easier [`time`] values.
    pub fn time(&self) -> PrimitiveDateTime {
        return from_vdif_time(self.epoch, self.time);
    }

    /// Return the station ID as either a string or a number.
    ///
    /// This function attempts to find two valid ASCII characters. If it fails it returns a number otherwise
    /// it returns a two character string. If you just want the numeric ID, use `self.station` instead.
    pub fn station(&self) -> StationID {
        let bytes = self.station.to_le_bytes();
        let id = String::from_utf8(bytes.to_vec());
        match id {
            Ok(idstring) => StationID::StringID(idstring),
            Err(_) => StationID::NumericID(self.station),
        }
    }

    /// Consume `self` and return a VDIF encoded array of bytes representing a header.
    pub fn encode(self) -> [u8; 32] {
        return encode_header(self)
    }
}

impl std::fmt::Display for VDIFHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut station: String = "  ".to_string();
        if let StationID::StringID(str) = self.station() {
            station = str
        }

        write!(f, "(Frame: {}, Thread: {}, Time: {}, Size: {}, Channels: {}, Bits/sample: {}, Real: {}, Valid: {}, Station: {} ({}))",
        self.frame, self.thread, self.time(), self.size, self.channels, self.bits_per_sample, self.is_real, self.is_valid, station, self.station)
    }
}

/// Convert a [`PrimitiveDateTime`] back to a raw VDIF `epoch` and `time`.
pub fn to_vdif_time(datetime: PrimitiveDateTime) -> (u8, u32) {
    let epoch: u8 = (((datetime.year() - 2000) as u8) * 12) / 6;

    let datetime_cpy = PrimitiveDateTime::new(
        Date::from_calendar_date(datetime.year(), time::Month::January, 1).unwrap(),
        Time::from_hms(0, 0, 0).unwrap(),
    );
    if epoch % 2 != 0 {
        // epoch is odd, so VDIF time starts on the year + 6 months
        datetime_cpy.replace_month(time::Month::July).unwrap();
    }

    let time = datetime - datetime_cpy;

    return (epoch, time.whole_seconds() as u32);
}

/// Convert a raw VDIF `epoch` and `time` to a [`PrimitiveDateTime`].
pub fn from_vdif_time(epoch: u8, time: u32) -> PrimitiveDateTime {
    let (years, month): (i32, time::Month) = match epoch % 2 == 0 {
        true => {
            // The ref epoch lands on a year
            ((epoch / 2) as i32, time::Month::January)
        }
        false => {
            // The ref epoch lands on a year + 6 months
            (((epoch - 1) / 2) as i32, time::Month::July)
        }
    };

    let date = Date::from_calendar_date(2000 + years, month, 1).unwrap();
    let dur = std::time::Duration::from_secs(time as u64);

    return PrimitiveDateTime::new(date, time::Time::from_hms(0, 0, 0).unwrap()) + dur;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_time_conversion() {
        let test_epoch = 6;
        let test_time = 167593;

        assert_eq!(
            (test_epoch, test_time),
            to_vdif_time(from_vdif_time(test_epoch, test_time))
        )
    }
}
