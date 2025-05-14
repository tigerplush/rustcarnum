use std::{array::TryFromSliceError, num::TryFromIntError, str::Utf8Error};

use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, Debug, TypePath)]
pub struct Dat {
    footer: DatFooter,
    entries: Vec<DatEntry>,
    raw: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum DatError {
    #[error("Error while slicing from buffer")]
    Slice(#[from] TryFromSliceError),
    #[error("Error while decoding")]
    InvalidLength,
    #[error("")]
    Utf8(#[from] Utf8Error),
    #[error("")]
    Int(#[from] TryFromIntError),
    #[error("")]
    NoFilenameEnd,
    #[error("")]
    NoValidDatEntryType,
}

impl Dat {
    pub(crate) fn from_buffer(buffer: &[u8]) -> Result<Dat, DatError> {
        let footer = DatFooter::from_buffer(&buffer[buffer.len() - DatFooter::SIZE..])?;
        info!("{:?}", footer);

        let filetable_start = buffer.len() - footer.dat_entry_start_from_end;
        let num_entries =
            u32::from_le_bytes(buffer[filetable_start..filetable_start + 4].try_into()?);
        let mut current_entry_ptr = filetable_start + 8;
        let mut entries = Vec::new();
        while current_entry_ptr < buffer.len() - DatFooter::SIZE {
            let entry = DatEntry::from_buffer(&buffer[current_entry_ptr..])?;
            current_entry_ptr += entry.len();
            entries.push(entry);
        }
        info!("loaded {} from {} entries", entries.len(), num_entries);
        Ok(Dat {
            footer,
            entries,
            raw: buffer.to_vec(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct DatFooter {
    uuid: [u8; 16],
    magic: String,
    filename_total_bytes: u32,
    dat_entry_start_from_end: usize,
}

impl DatFooter {
    const SIZE: usize = 28;
    fn from_buffer(buffer: &[u8]) -> Result<DatFooter, DatError> {
        if buffer.len() != DatFooter::SIZE {
            return Err(DatError::InvalidLength);
        }
        let uuid = buffer[0..16].try_into()?;
        let magic = str::from_utf8(&buffer[16..20])?.to_string();
        let filename_total_bytes = u32::from_le_bytes(buffer[20..24].try_into()?);
        let dat_entry_start_from_end = u32::from_le_bytes(buffer[24..28].try_into()?).try_into()?;
        Ok(DatFooter {
            uuid,
            magic,
            filename_total_bytes,
            dat_entry_start_from_end,
        })
    }
}

#[derive(Debug)]
pub enum DatEntryType {
    Stored,
    Compressed,
    Directory,
}

#[derive(Debug)]
pub struct DatEntry {
    filename: String,
    unk_value: u32,
    pub entry_type: DatEntryType,
    pub original_size: usize,
    pub deflate_size: usize,
    pub offset: usize,
    mystery_number: u32,
}

impl DatEntry {
    fn from_buffer(buffer: &[u8]) -> Result<DatEntry, DatError> {
        let filename_end = buffer
            .iter()
            .position(|&byte| byte == 0)
            .ok_or(DatError::NoFilenameEnd)?;
        let filename = str::from_utf8(&buffer[0..filename_end])?.into();
        let mut values = [0; 6];
        for i in 0..values.len() {
            let start = i * 4;
            let end = start + 4;
            values[i] = u32::from_le_bytes(
                buffer[filename_end + 1 + start..filename_end + 1 + end].try_into()?,
            );
        }
        let entry_type = match values[1] {
            0x01 => DatEntryType::Stored,
            0x02 => DatEntryType::Compressed,
            0x0400 => DatEntryType::Directory,
            _ => return Err(DatError::NoValidDatEntryType),
        };
        Ok(DatEntry {
            filename,
            unk_value: values[0],
            entry_type,
            original_size: values[2].try_into()?,
            deflate_size: values[3].try_into()?,
            offset: values[4].try_into()?,
            mystery_number: values[5],
        })
    }

    fn len(&self) -> usize {
        self.filename.len() + 1 + 24
    }
}
