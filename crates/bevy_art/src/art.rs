use std::{array::TryFromSliceError, ops::AddAssign};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use serde::Deserialize;
use thiserror::Error;

/// Intermediate struct to decode the custom `.ART` format Arcanum uses.
/// Since the .ART format contains multiple frames, these frames will be rendered into one image
/// along the x-axis. Simultaneously, a TextureAtlas will be built from the meta infos
#[allow(dead_code)]
#[derive(Asset, Debug, Deserialize, TypePath)]
pub struct Art {
    data: Option<Vec<u8>>,
    header: ArtHeader,
    color_table_data: Vec<ColorTable>,
    frame_data: Vec<ArtFrame>,
}

pub type TigArtId = u32;
const ART_ID_TYPE_SHIFT: u32 = 28;

pub const ART_TYPE_INTERFACE: u32 = 5;
const INTERFACE_ID_MAX_NUM: u32 = 4096;
const INTERFACE_ID_NUM_SHIFT: u32 = 16;

pub fn art_id_create(num: u32, frame: u32, a3: u8, palette: u32) -> Result<TigArtId, ()> {
    let art_id = (ART_TYPE_INTERFACE << ART_ID_TYPE_SHIFT)
        | ((num & (INTERFACE_ID_MAX_NUM - 1)) << INTERFACE_ID_NUM_SHIFT);
    Ok(art_id)
}

pub fn art_type(art_id: TigArtId) -> u32 {
    art_id >> ART_ID_TYPE_SHIFT
}

#[test]
fn test_interface() {
    // assert_eq!(art_id_create(327, 0, 0, 0), Ok(u32::MAX));
    assert_eq!(
        art_type(art_id_create(327, 0, 0, 0).unwrap()),
        ART_TYPE_INTERFACE
    );
}

#[derive(Debug, Error)]
pub enum ArtError {
    #[error("Error while slicing from buffer")]
    Slice(#[from] TryFromSliceError),
    #[error("FrameData contained 0 frames")]
    EmptyFrame,
}

impl Art {
    pub fn from_buffer(buffer: &[u8]) -> Result<Art, ArtError> {
        let header = ArtHeader::from_buffer(&buffer[0..ArtHeader::SIZE])?;
        let palettes = header
            .stupid_color
            .iter()
            .filter(|&color| color.in_palette())
            .count();
        let mut current_index = ArtHeader::SIZE;
        let mut color_table_data = Vec::new();
        for _ in 0..palettes {
            let color_table =
                ColorTable::from_buffer(&buffer[current_index..current_index + ColorTable::SIZE])?;
            color_table_data.push(color_table);
            current_index += ColorTable::SIZE;
        }

        let mut frame_data = Vec::new();
        for _ in 0..header.frames() {
            let frame = ArtFrame::header_from_buffer(
                &buffer[current_index..current_index + ArtFrameHeader::SIZE],
            )?;
            frame_data.push(frame);
            current_index += ArtFrameHeader::SIZE;
        }

        for frame in &mut frame_data {
            frame.load_pixels_from_buffer(&buffer[current_index..current_index + frame.size()])?;
            current_index += frame.size();
        }

        Ok(Art {
            data: None,
            header,
            color_table_data,
            frame_data,
        })
    }

    pub fn to_image(&self) -> Result<Image, ArtError> {
        // accumulate width, so we can place all frames next to each other
        let width = self
            .frame_data
            .iter()
            .fold(0, |acc, frame| acc + frame.header.width);
        // check for highest frame
        let height = self
            .frame_data
            .iter()
            .map(|frame| frame.header.height)
            .max()
            .ok_or(ArtError::EmptyFrame)?;
        info!("w {}, h {}", width, height);
        let mut data = vec![0; width as usize * height as usize * 4];
        let mut offset = 0;
        for frame in &self.frame_data {
            for y in (0..frame.header.height as usize).rev() {
                for x in 0..frame.width() {
                    let sample = frame.pixels[y][x] as usize;
                    let target_x = x + offset;
                    let index = (y * width as usize + target_x) * 4;
                    match sample {
                        0 => {
                            data[index + 3] = 0;
                        }
                        _ => {
                            let color = &self.color_table_data[0].0[sample];
                            data[index] = color.r;
                            data[index + 1] = color.g;
                            data[index + 2] = color.b;
                            data[index + 3] = color.opacity();
                        }
                    };
                }
            }
            offset += frame.width();
        }
        let image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(),
        );
        Ok(image)
    }

    pub fn to_texture_atlas(&self) -> TextureAtlasLayout {
        let columns = self.frame_data.len() as u32;
        let mut textures = vec![];
        let mut current_x = 0;
        for frame in &self.frame_data {
            textures.push(URect::new(
                current_x,
                frame.header.height,
                current_x + frame.header.width,
                0,
            ));
            current_x += frame.header.width;
        }
        TextureAtlasLayout {
            size: UVec2::new(1, columns),
            textures,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ArtHeader {
    h0: [u32; 3],
    stupid_color: [Color; 4],

    frame_num_low: u32,
    frame_num: u32,
    palette_data1: ColorPalette,
    palette_data2: ColorPalette,
    palette_data3: ColorPalette,
}

impl ArtHeader {
    const SIZE: usize = 132;
    fn from_buffer(buffer: &[u8]) -> Result<ArtHeader, ArtError> {
        let h1 = u32::from_le_bytes(buffer[0..4].try_into()?);
        let h2 = u32::from_le_bytes(buffer[4..8].try_into()?);
        let h3 = u32::from_le_bytes(buffer[8..12].try_into()?);
        let stupid_color0 = Color::from_buffer(&buffer[12..16])?;
        let stupid_color1 = Color::from_buffer(&buffer[16..20])?;
        let stupid_color2 = Color::from_buffer(&buffer[20..24])?;
        let stupid_color3 = Color::from_buffer(&buffer[24..28])?;
        let frame_num_low = u32::from_le_bytes(buffer[28..32].try_into()?);
        let frame_num = u32::from_le_bytes(buffer[32..36].try_into()?);
        let palette_data1 = ColorPalette::from_buffer(&buffer[36..68])?;
        let palette_data2 = ColorPalette::from_buffer(&buffer[68..100])?;
        let palette_data3 = ColorPalette::from_buffer(&buffer[100..132])?;
        Ok(ArtHeader {
            h0: [h1, h2, h3],
            stupid_color: [stupid_color0, stupid_color1, stupid_color2, stupid_color3],
            frame_num_low,
            frame_num,
            palette_data1,
            palette_data2,
            palette_data3,
        })
    }

    fn animated(&self) -> bool {
        self.h0[0] & 0x1 == 0
    }

    fn frames(&self) -> u32 {
        match self.animated() {
            true => self.frame_num * 8,
            false => self.frame_num,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    /// alpha of the color, is 255 - opacity
    pub a: u8,
}

impl Color {
    fn from_buffer(buffer: &[u8]) -> Result<Color, ArtError> {
        let b = u8::from_le_bytes(buffer[0..1].try_into()?);
        let g = u8::from_le_bytes(buffer[1..2].try_into()?);
        let r = u8::from_le_bytes(buffer[2..3].try_into()?);
        let a = u8::from_le_bytes(buffer[3..4].try_into()?);
        Ok(Color { b, g, r, a })
    }

    fn in_palette(&self) -> bool {
        self.b | self.g | self.r | self.a != 0
    }

    fn opacity(&self) -> u8 {
        255 - self.a
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ColorPalette([Color; 8]);

impl ColorPalette {
    fn from_buffer(buffer: &[u8]) -> Result<ColorPalette, ArtError> {
        let mut colors = Vec::new();
        for i in 0..8 {
            let index = i * 4;
            colors.push(Color::from_buffer(&buffer[index..index + 4])?);
        }
        Ok(ColorPalette(colors.try_into().unwrap()))
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ColorTable(Vec<Color>);

impl ColorTable {
    const SIZE: usize = 1024;
    fn from_buffer(buffer: &[u8]) -> Result<ColorTable, ArtError> {
        let mut colors = Vec::new();
        for i in 0..256 {
            let index = i * 4;
            colors.push(Color::from_buffer(&buffer[index..index + 4])?);
        }
        Ok(ColorTable(colors))
    }
}

#[derive(Debug, Deserialize)]
struct ArtFrame {
    header: ArtFrameHeader,
    pixels: Vec<Vec<u8>>,
}

impl ArtFrame {
    fn header_from_buffer(buffer: &[u8]) -> Result<ArtFrame, ArtError> {
        let header = ArtFrameHeader::from_buffer(buffer)?;
        Ok(ArtFrame {
            header,
            pixels: Vec::new(),
        })
    }

    fn load_pixels_from_buffer(&mut self, buffer: &[u8]) -> Result<(), ArtError> {
        let size = self.header.height * self.header.width;
        self.pixels = vec![vec![0; self.header.width as usize]; self.header.height as usize];
        if self.header.size < size as usize {
            let mut counter = Counter::new(self.header.width as usize, self.header.height as usize);
            let mut p = 0;
            while p < self.header.size {
                match buffer[p] & 0x80 {
                    0x80 => {
                        let copies = buffer[p] & 0x7F;
                        for _ in 0..copies {
                            p += 1;
                            self.pixels[counter.y][counter.x] = buffer[p];
                            counter += 1;
                        }
                    }
                    _ => {
                        let clones = buffer[p] & 0x7F;
                        p += 1;
                        let val = buffer[p];
                        for _ in 0..clones {
                            self.pixels[counter.y][counter.x] = val;
                            counter += 1;
                        }
                    }
                }
                p += 1;
            }
        } else {
            for p in 0..self.header.size {
                let x = p % self.header.width as usize;
                let y = p / self.header.width as usize;
                self.pixels[y][x] = buffer[p];
            }
        }
        Ok(())
    }

    fn size(&self) -> usize {
        self.header.size
    }

    fn width(&self) -> usize {
        self.header.width as usize
    }
}

struct Counter {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Counter {
    const fn new(width: usize, height: usize) -> Counter {
        Counter {
            x: 0,
            y: 0,
            width,
            height,
        }
    }
}

impl AddAssign<usize> for Counter {
    fn add_assign(&mut self, rhs: usize) {
        self.x += rhs;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y > self.height {
            panic!("coordinates are out of bounds");
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ArtFrameHeader {
    width: u32,
    height: u32,
    size: usize,
    c_x: i32,
    c_y: i32,
    d_x: i32,
    d_y: i32,
}

impl ArtFrameHeader {
    const SIZE: usize = 28;
    fn from_buffer(buffer: &[u8]) -> Result<ArtFrameHeader, ArtError> {
        let width = u32::from_le_bytes(buffer[0..4].try_into()?);
        let height = u32::from_le_bytes(buffer[4..8].try_into()?);
        let size = u32::from_le_bytes(buffer[8..12].try_into()?) as usize;
        let c_x = i32::from_le_bytes(buffer[12..16].try_into()?);
        let c_y = i32::from_le_bytes(buffer[16..20].try_into()?);
        let d_x = i32::from_le_bytes(buffer[20..24].try_into()?);
        let d_y = i32::from_le_bytes(buffer[24..28].try_into()?);
        Ok(ArtFrameHeader {
            width,
            height,
            size,
            c_x,
            c_y,
            d_x,
            d_y,
        })
    }
}
