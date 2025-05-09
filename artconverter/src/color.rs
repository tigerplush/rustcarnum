#[derive(Debug)]
pub struct Color {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

impl TryFrom<&[u8]> for Color {
    type Error = std::array::TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let b = u8::from_le_bytes(value[0..1].try_into()?);
        let g = u8::from_le_bytes(value[1..2].try_into()?);
        let r = u8::from_le_bytes(value[2..3].try_into()?);
        let a = u8::from_le_bytes(value[3..4].try_into()?);
        Ok(Color { b, g, r, a })
    }
}

#[derive(Debug)]
pub struct ColorPalette(pub [Color; 8]);

impl TryFrom<&[u8]> for ColorPalette {
    type Error = std::array::TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut colors = Vec::new();
        for i in 0..8 {
            let index = i * 4;
            colors.push(Color::try_from(&value[index..index + 4])?);
        }
        Ok(ColorPalette(colors.try_into().unwrap()))
    }
}

pub fn in_palette(color: &Color) -> bool {
    color.a | color.b | color.g | color.r != 0
}
