use std::{fmt::Display, fs::File, io::Read};

use crate::{
    artconverter_error::ArtconverterError,
    color::{Color, ColorPalette},
};

#[derive(Debug)]
pub struct ArtHeader {
    h0: [u32; 3],
    pub stupid_color: [Color; 4],

    pub frame_num_low: u32,
    pub frame_num: u32,
    pub palette_data1: ColorPalette,
    palette_data2: ColorPalette,
    palette_data3: ColorPalette,
}

impl ArtHeader {
    pub fn animated(&self) -> bool {
        self.h0[0] & 0x1 == 0
    }
}

impl Display for ArtHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ArtHeader {{\n\th0: {:?}\n\tstupid_color: {:?}\n\tframe_num_low: {}\n\tframe_num: {}\n}}",
            self.h0, self.stupid_color, self.frame_num_low, self.frame_num
        )
    }
}

impl TryFrom<&mut File> for ArtHeader {
    type Error = ArtconverterError;
    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let mut header_buf = [0; 132];
        file.read_exact(&mut header_buf)?;
        ArtHeader::try_from(header_buf.as_slice())
    }
}

impl TryFrom<&[u8]> for ArtHeader {
    type Error = ArtconverterError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let h1 = u32::from_le_bytes(value[0..4].try_into()?);
        let h2 = u32::from_le_bytes(value[4..8].try_into()?);
        let h3 = u32::from_le_bytes(value[8..12].try_into()?);
        let stupid_color0 = Color::try_from(&value[12..16])?;
        let stupid_color1 = Color::try_from(&value[16..20])?;
        let stupid_color2 = Color::try_from(&value[20..24])?;
        let stupid_color3 = Color::try_from(&value[24..28])?;
        let frame_num_low = u32::from_le_bytes(value[28..32].try_into()?);
        let frame_num = u32::from_le_bytes(value[32..36].try_into()?);
        Ok(ArtHeader {
            h0: [h1, h2, h3],
            stupid_color: [stupid_color0, stupid_color1, stupid_color2, stupid_color3],
            frame_num_low,
            frame_num,
            palette_data1: ColorPalette::try_from(&value[36..68])?,
            palette_data2: ColorPalette::try_from(&value[68..100])?,
            palette_data3: ColorPalette::try_from(&value[100..132])?,
        })
    }
}
