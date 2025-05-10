use std::ops::Range;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DatFooter {
    uuid: [u8; 16],
    magic: String,
    filename_total_bytes: u32,
    pub dat_entry_start_from_end: usize,
}

impl DatFooter {
    pub const LENGTH: usize = 28;
    const UUID_RANGE: Range<usize> = 0..16;
    const MAGIC_RANGE: Range<usize> = 16..20;
    const FILENAME_RANGE: Range<usize> = 20..24;
    const DAT_ENTRY_RANGE: Range<usize> = 24..28;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum DatFooterError {
    InvalidLength,
    SliceConversion(std::array::TryFromSliceError),
    Utf8Error(std::str::Utf8Error),
}

impl From<std::array::TryFromSliceError> for DatFooterError {
    fn from(err: std::array::TryFromSliceError) -> Self {
        DatFooterError::SliceConversion(err)
    }
}

impl From<std::str::Utf8Error> for DatFooterError {
    fn from(err: std::str::Utf8Error) -> Self {
        DatFooterError::Utf8Error(err)
    }
}

impl TryFrom<&[u8]> for DatFooter {
    type Error = DatFooterError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != DatFooter::LENGTH {
            return Err(DatFooterError::InvalidLength);
        }
        let uuid: [u8; 16] = value[DatFooter::UUID_RANGE].try_into()?;
        let magic = str::from_utf8(&value[DatFooter::MAGIC_RANGE])?.to_string();
        let filename_total_bytes_slice: [u8; 4] = value[DatFooter::FILENAME_RANGE].try_into()?;
        let dat_entry_start_from_end_slice: [u8; 4] =
            value[DatFooter::DAT_ENTRY_RANGE].try_into()?;
        Ok(DatFooter {
            uuid,
            magic,
            filename_total_bytes: u32::from_le_bytes(filename_total_bytes_slice),
            dat_entry_start_from_end: u32::from_le_bytes(dat_entry_start_from_end_slice) as usize,
        })
    }
}
