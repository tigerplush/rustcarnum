use std::{fs::File, path::Path};

use rustbitmap::{BitMap, Rgba};

use crate::{
    art_frame::ArtFrame, art_header::ArtHeader, artconverter_error::ArtconverterError,
    color::in_palette, color_table::ColorTable,
};

pub struct ArtFile {
    header: ArtHeader,
    palette_data: Vec<ColorTable>,
    frame_data: Vec<ArtFrame>,
}

impl ArtFile {
    pub fn load_from_file(input_filepath: String) -> Result<Self, ArtconverterError> {
        // let data: Vec<u8> = fs::read(input_filepath)?;
        let file = File::open(input_filepath)?;
        ArtFile::try_from(file)
    }

    pub fn save_as_bmp(&self, output_filepath: String) -> Result<(), ArtconverterError> {
        for frame in &self.frame_data {
            let mut bitmap = BitMap::new(frame.header.width, frame.header.height);
            for y in (0..frame.header.height).rev() {
                for x in 0..frame.header.width {
                    let index = (y * frame.header.width + x) * 3;
                    let red = frame.pixels[index as usize];
                    let green = frame.pixels[index as usize + 1];
                    let blue = frame.pixels[index as usize + 2];
                    bitmap.set_pixel(x, y, Rgba::rgb(red, green, blue))?;
                }
            }
            let path = Path::new(&output_filepath).join("test.bmp");
            bitmap.save_as(path.to_str().unwrap())?;
        }
        Ok(())
    }
}

impl TryFrom<File> for ArtFile {
    type Error = ArtconverterError;
    fn try_from(mut file: File) -> Result<Self, Self::Error> {
        let header = ArtHeader::try_from(&mut file)?;

        let mut palettes = 0;
        for color in &header.stupid_color {
            if in_palette(color) {
                palettes += 1;
            }
        }
        let mut frames = header.frame_num;
        if header.animated() {
            frames *= 8;
        }

        let mut palette_data = Vec::new();
        for _ in 0..palettes {
            let color_table = ColorTable::try_from(&mut file)?;
            palette_data.push(color_table);
        }
        println!("loaded {} palettes", palette_data.len());

        let mut frame_data = Vec::new();
        for _ in 0..frames {
            let frame = ArtFrame::try_from(&mut file)?;
            println!("{}", frame);
            frame_data.push(frame);
        }

        for frame in &mut frame_data {
            frame.load(&mut file)?;
            println!("{}", frame);
        }

        Ok(ArtFile {
            header,
            palette_data,
            frame_data,
        })
    }
}
