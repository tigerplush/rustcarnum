use std::{fs::File, path::Path};

use bmp_rust::bmp::BMP;

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
        // let width = self.frame_data.iter().map(|frame| frame.header.width).sum::<u32>();
        // let height = self.frame_data.iter().map(|frame| frame.header.height).max().unwrap();
        for (index, frame) in self.frame_data.iter().enumerate() {
            let mut bitmap = BMP::new(frame.header.height as i32, frame.header.width, None);
            for y in (0..frame.header.height).rev() {
                for x in 0..frame.header.width {
                    let px = x as usize;
                    let py = y as usize;
                    let value = frame.pixels[py][px];
                    let col = &self.palette_data[0].0[value as usize];
                    bitmap.change_color_of_pixel(
                        x as u16,
                        y as u16,
                        [col.r, col.g, col.b, u8::MAX],
                    )?;
                }
            }
            let path = Path::new(&output_filepath).join(format!("test_{}.bmp", index));
            bitmap.save_to_new(path.to_str().unwrap())?;
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
            // println!("{:?}", frame.pixels);
        }

        Ok(ArtFile {
            header,
            palette_data,
            frame_data,
        })
    }
}
