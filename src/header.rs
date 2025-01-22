use chrono::{
    naive::{NaiveDate, NaiveDateTime},
    Datelike, NaiveTime, TimeDelta,
};

/// VDIF station identifiers can be either a two character ASCII string, or a numeric ID.
pub enum StationID {
    /// The station ID as a two character ASCII string
    StringID(String),
    /// The station ID as a number
    NumericID(u16),
}

impl StationID {
    /// Encode this station ID into a `u16` VDIF header field.
    pub fn encode(self) -> u16 {
        match self {
            Self::StringID(s) => {
                u16::from_be_bytes(s.as_bytes().try_into().expect(
                    "Tried to encode a StationID with more/less than two ASCII characters!",
                ))
            }
            Self::NumericID(id) => id,
        }
    }
}

/// A VDIF data frame header.
///
/// The header information is accessed through public fields and methods.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
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
    pub frameno: u32,
    /// The VDIF version.
    pub version: u8,
    /// The number of channels within the frame stored as 2<sup># Channels</sup>.
    pub channels: u8,
    /// The size in units of 8 bytes of the data frame (header **and** payload).
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
    /// Get the total size in bytes of the associated VDIF frame.
    pub fn bytesize(&self) -> u32 {
        return self.size * 8;
    }

    /// Get the total size in 32-bit words of the associated VDIF frame.
    pub fn wordsize(&self) -> u32 {
        return self.bytesize() / 4;
    }

    /// Get the total size in bytes of the associated VDIF payload.
    pub fn data_bytesize(&self) -> u32 {
        return self.bytesize() - 32;
    }

    /// Get the total size in 32-bit words of the associated VDIF payload.
    pub fn data_wordsize(&self) -> u32 {
        return (self.bytesize() - 32) / 4;
    }

    /// Get the number of channels contained within the associated VDIF payload.
    pub fn channelno(&self) -> usize {
        return 1usize << self.channels;
    }

    /// Get a [`NaiveDateTime`] representing the `epoch` and `time` of the associated VDIF frame.
    pub fn date(&self) -> NaiveDateTime {
        return vdiftime_to_date(self.epoch, self.time);
    }

    /// Return the station ID as either a string or a number.
    ///
    /// This function attempts to find two valid ASCII characters in the station ID field. If it fails it returns a number, otherwise
    /// it returns a two character string. If you know you just want the numeric ID, use `self.station` instead.
    pub fn station(&self) -> StationID {
        // FIXME: double check this actually works on all systems, (I fudged this)
        let bytes = self.station.to_be_bytes();
        let id = String::from_utf8(bytes.to_vec());
        match id {
            Ok(idstring) => StationID::StringID(idstring),
            Err(_) => StationID::NumericID(self.station),
        }
    }
}

impl std::fmt::Display for VDIFHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut station: String = "  ".to_string();
        if let StationID::StringID(str) = self.station() {
            station = str
        }

        write!(f, "(Frame: {}, Thread: {}, Time: {}, Size: {}, Channels: {}, Bits/sample: {}, Real: {}, Valid: {}, Station: {} ({}))",
        self.frameno, self.thread, self.time, self.size*8, 1 << self.channels, self.bits_per_sample, self.is_real, self.is_valid, station, self.station)
    }
}

/// Convert a VDIF `epoch` and `time` value to a [`NaiveDateTime`] from the [`chrono`] library.
pub fn vdiftime_to_date(epoch: u8, time: u32) -> NaiveDateTime {
    let years = epoch / 2;
    let months = if epoch % 2 > 0 { 7 } else { 1 };
    let delta = TimeDelta::new(time as i64, 0).expect("Incorrect time supplied to chrono");

    return NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2000 + years as i32, months as u32, 1)
            .expect("Incorrect epoch supplied to chrono"),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    ) + delta;
}

/// Convert a [`NaiveDateTime`] from the [`chrono`] library to a VDIF `epoch` and `time`.
pub fn vdiftime_from_date(datetime: NaiveDateTime) -> (u8, u32) {
    let epoch_month = if datetime.month() > 6 { 7 } else { 1 };
    let epoch_date = NaiveDate::from_ymd_opt(datetime.year(), epoch_month, 1).unwrap();
    let time = datetime - NaiveDateTime::new(epoch_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());

    let mut epoch = (datetime.year() - 2000) * 2;
    if datetime.month() > 6 {
        epoch += 1
    };

    return (epoch as u8, time.num_seconds() as u32);
}

#[cfg(test)]
mod tests {
    use super::StationID;

    #[test]
    fn test_stationid_encode() {
        let testid = StationID::NumericID(12345);
        assert_eq!(testid.encode(), 12345);

        let teststr = StationID::StringID("JB".to_owned());
        assert_eq!(teststr.encode(), 0b0100101001000010)
    }
}
