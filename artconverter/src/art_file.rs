use std::fs::File;

use crate::{
    art_header::ArtHeader, artconverter_error::ArtconverterError, color::in_palette,
    color_table::ColorTable,
};

pub struct ArtFile {
    header: ArtHeader,
    palette_data: Vec<ColorTable>,
}

impl ArtFile {
    pub fn load_from_file(input_filepath: String) -> Result<Self, ArtconverterError> {
        // let data: Vec<u8> = fs::read(input_filepath)?;
        let file = File::open(input_filepath)?;
        ArtFile::try_from(file)
    }

    pub fn save_as_bmp(&self, output_filepath: String) {}
}

impl TryFrom<File> for ArtFile {
    type Error = ArtconverterError;
    fn try_from(mut file: File) -> Result<Self, Self::Error> {
        let header = ArtHeader::try_from(&mut file)?;
        println!("{}", header);

        let mut palettes = 0;
        for color in &header.stupid_color {
            if in_palette(color) {
                palettes += 1;
            }
        }
        let mut frames = header.frame_num;
        let key_frame = header.frame_num_low;
        if header.animated() {
            frames *= 8;
        }

        let mut palette_data = Vec::new();
        for i in 0..palettes {
            let color_table = ColorTable::try_from(&mut file)?;
            palette_data.push(color_table);
        }
        println!("loaded {} palettes", palette_data.len());

        Ok(ArtFile {
            header,
            palette_data,
        })
    }
}
