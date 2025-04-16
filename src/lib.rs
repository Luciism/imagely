mod manipulation;

use core::panic;
use std::fs;

#[derive(Debug)]
pub struct PixelRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl PixelRGBA {
    pub fn build(data: Vec<u8>) -> PixelRGBA {
        PixelRGBA {
            r: data[0],
            g: data[1],
            b: data[2],
            a: data[3],
        }
    }
}

#[derive(Debug)]
pub struct PixelRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PixelRGB {
    pub fn build(data: Vec<u8>) -> PixelRGB {
        PixelRGB {
            r: data[0],
            g: data[1],
            b: data[2],
        }
    }
}

#[derive(Debug)]
pub enum Pixel {
    RGBA(PixelRGBA),
    RGB(PixelRGB),
}

#[derive(Debug)]
pub struct Image {
    pub channels: u8,
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

impl Image {
    pub fn load_binary_image(
        path: &str,
        channels: u8,
        width: u32,
        height: u32,
    ) -> Result<Image, std::io::Error> {
        let contents = fs::read(path)?;
        Ok(Image {
            channels,
            width,
            height,
            data: contents,
        })
    }

    pub fn write_binary_image(self, path: &str) -> Result<(), std::io::Error> {
        fs::write(path, self.data)
    }
}

pub struct PixelIterator<'a> {
    image: &'a Image,
    cur_pixel_index: usize,
}

impl<'a> PixelIterator<'a> {
    pub fn new(image: &'a Image) -> Self {
        PixelIterator {
            image,
            cur_pixel_index: 0,
        }
    }
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let channels = self.image.channels;

        let greatest_pixel_data_index =
            self.cur_pixel_index * channels as usize + channels as usize;
        if greatest_pixel_data_index >= self.image.data.len() {
            return None;
        }

        let pixel_data =
            &self.image.data[(self.cur_pixel_index * channels as usize)..greatest_pixel_data_index];
        self.cur_pixel_index += 1;

        if channels == 4 {
            return Some(Pixel::RGBA(PixelRGBA::build(pixel_data.to_vec())));
        } else if channels == 3 {
            return Some(Pixel::RGB(PixelRGB::build(pixel_data.to_vec())));
        }
        panic!("[FATAL] Invalid number of channels: {}", channels);
    }
}
