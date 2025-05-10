use std::{fs::File, io::Read};

use crate::{artconverter_error::ArtconverterError, color::Color};

#[derive(Debug)]
pub struct ColorTable(pub [Color; 256]);

impl TryFrom<&mut File> for ColorTable {
    type Error = ArtconverterError;
    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let mut color_table_buf = [0; 256 * 4];
        file.read_exact(&mut color_table_buf)?;
        let mut colors = Vec::new();
        for i in 0..256 {
            let index = i * 4;
            colors.push(Color::try_from(&color_table_buf[index..index + 4])?);
        }
        Ok(ColorTable(colors.try_into().unwrap()))
    }
}
