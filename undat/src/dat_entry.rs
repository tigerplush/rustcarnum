#[derive(Debug)]
pub struct DatEntry {
    pub filepath: String,
    unk_value: u32,
    pub entry_type: DatEntryType,
    pub original_size: usize,
    pub deflate_size: usize,
    pub offset: usize,
    mystery_number: u32,
}

impl DatEntry {
    pub fn len(&self) -> usize {
        self.filepath.len() + 1 + 24
    }
}

#[derive(Debug)]
pub enum DatEntryType {
    Stored,
    Compressed,
    Directory,
}

#[derive(Debug)]
pub enum DatEntryError {
    NoFilenameEnd,
    FilenameNotUtf8(std::str::Utf8Error),
    NoValidDatEntryType,
    SliceConversion(std::array::TryFromSliceError),
}

impl From<std::str::Utf8Error> for DatEntryError {
    fn from(err: std::str::Utf8Error) -> Self {
        DatEntryError::FilenameNotUtf8(err)
    }
}

impl From<std::array::TryFromSliceError> for DatEntryError {
    fn from(err: std::array::TryFromSliceError) -> Self {
        DatEntryError::SliceConversion(err)
    }
}

impl TryFrom<&[u8]> for DatEntry {
    type Error = DatEntryError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let filename_end = value
            .iter()
            .position(|&byte| byte == 0)
            .ok_or(DatEntryError::NoFilenameEnd)?;
        let filename = str::from_utf8(&value[0..filename_end])?.to_string();

        let unk_value_slice: [u8; 4] = value[filename_end + 1..filename_end + 1 + 4].try_into()?;
        let entry_type_slice: [u8; 4] =
            value[filename_end + 1 + 4..filename_end + 1 + 8].try_into()?;
        let original_size_slice: [u8; 4] =
            value[filename_end + 1 + 8..filename_end + 1 + 12].try_into()?;
        let deflate_size_slice: [u8; 4] =
            value[filename_end + 1 + 12..filename_end + 1 + 16].try_into()?;
        let offset_slice: [u8; 4] =
            value[filename_end + 1 + 16..filename_end + 1 + 20].try_into()?;
        let mystery_slice: [u8; 4] =
            value[filename_end + 1 + 20..filename_end + 1 + 24].try_into()?;

        let entry_type = match u32::from_le_bytes(entry_type_slice) {
            0x01 => DatEntryType::Stored,
            0x02 => DatEntryType::Compressed,
            0x0400 => DatEntryType::Directory,
            _ => return Err(DatEntryError::NoValidDatEntryType),
        };

        Ok(DatEntry {
            filepath: filename,
            unk_value: u32::from_le_bytes(unk_value_slice),
            entry_type,
            original_size: u32::from_le_bytes(original_size_slice) as usize,
            deflate_size: u32::from_le_bytes(deflate_size_slice) as usize,
            offset: u32::from_le_bytes(offset_slice) as usize,
            mystery_number: u32::from_le_bytes(mystery_slice),
        })
    }
}
