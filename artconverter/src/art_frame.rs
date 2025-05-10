use std::{fs::File, io::Read};

use crate::artconverter_error::ArtconverterError;

#[derive(Debug)]
pub struct ArtFrame {
    pub header: ArtFrameHeader,
    pub pixels: Vec<Vec<u8>>,
}

impl ArtFrame {
    pub fn load(&mut self, file: &mut File) -> Result<(), ArtconverterError> {
        let mut data = vec![0; self.header.size];
        file.read_exact(&mut data)?;
        let size = self.header.height * self.header.width;
        self.pixels = vec![vec![0; self.header.width as usize]; self.header.height as usize];
        let mut x = 0;
        let mut y = 0;
        if self.header.size < size as usize {
            let mut p = 0;
            while p < self.header.size {
                println!("p is {}", p);
                match data[p] & 0x80 {
                    0x80 => {
                        let copies = data[p] & 0x7F;
                        for _ in 0..copies {
                            p += 1;
                            self.pixels[y][x] = data[p];
                            x += 1;
                            if x >= self.header.width as usize {
                                x = 0;
                                y += 1;
                            }
                        }
                    }
                    _ => {
                        let clones = data[p] & 0x7F;
                        p += 1;
                        let val = data[p];
                        for _ in 0..clones {
                            self.pixels[y][x] = val;

                            x += 1;
                            if x >= self.header.width as usize {
                                x = 0;
                                y += 1;
                            }
                        }
                    }
                }
                p += 1;
            }
        } else {
            for p in 0..self.header.size {
                self.pixels[y][x] = data[p];

                x += 1;
                if x >= self.header.width as usize {
                    x = 0;
                    y += 1;
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for ArtFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ArtFrame {{\n\theader: {:?}\n\tpixels: {}\n}}",
            self.header,
            self.pixels.len()
        )
    }
}

impl TryFrom<&mut File> for ArtFrame {
    type Error = ArtconverterError;
    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let header = ArtFrameHeader::try_from(file)?;
        Ok(ArtFrame {
            header,
            pixels: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct ArtFrameHeader {
    pub width: u32,
    pub height: u32,
    size: usize,
    c_x: i32,
    c_y: i32,
    d_x: i32,
    d_y: i32,
}

impl TryFrom<&mut File> for ArtFrameHeader {
    type Error = ArtconverterError;
    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let mut art_frame_header_buf = [0; 28];
        file.read_exact(&mut art_frame_header_buf)?;
        let width = u32::from_le_bytes(art_frame_header_buf[0..4].try_into()?);
        let height = u32::from_le_bytes(art_frame_header_buf[4..8].try_into()?);
        let size = u32::from_le_bytes(art_frame_header_buf[8..12].try_into()?) as usize;
        let c_x = i32::from_le_bytes(art_frame_header_buf[12..16].try_into()?);
        let c_y = i32::from_le_bytes(art_frame_header_buf[16..20].try_into()?);
        let d_x = i32::from_le_bytes(art_frame_header_buf[20..24].try_into()?);
        let d_y = i32::from_le_bytes(art_frame_header_buf[24..28].try_into()?);
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
