mod manipulation;
mod geometry;
mod filters;

use core::panic;
use std::{fs, ops::Add};

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

    pub fn pos_to_index(&self, x: usize, y: usize) -> usize {
        (y * self.width as usize + x) * self.channels as usize
    }

    pub fn index_to_pos(&self, index: usize) -> (usize, usize) {
        let pixel_index = index / self.channels as usize;
        self.pixel_index_to_pos(pixel_index)
    }

    pub fn pixel_index_to_index(&self, pixel_index: usize) -> usize {
        pixel_index * self.channels as usize
    }

    pub fn pixel_index_to_pos(&self, pixel_index: usize) -> (usize, usize) {
        let x = pixel_index % self.width as usize;
        let y = pixel_index / self.width as usize;

        (x, y)       
    }

    pub fn is_pos_in_image(&self, pos: (isize, isize)) -> bool {
        if pos.0 < 0 || pos.1 < 0 {
            return false;
        }

        if pos.0 >= self.width as isize || pos.1 >= self.height as isize {
            return false;
        }

        true
    }

    pub fn sum_pos<T: Add>(pos1: (T, T), pos2: (T, T)) -> (T::Output, T::Output) {
        (pos1.0 + pos2.0, pos1.1 + pos2.1)
    }

    pub fn subtract_pos(pos1: (usize, usize), pos2: (usize, usize)) -> (isize, isize) {
        (pos1.0 as isize - pos2.0 as isize, pos1.1 as isize - pos2.1 as isize)
    }

    pub fn new(width: u32, height: u32) -> Image {
        let channels: u8 = 4;
        let data_len = (width * height * channels as u32) as usize;

        let mut data = Vec::with_capacity(data_len);
        data.resize(data_len, 0);

        Image {
            channels,
            width,
            height,
            data
        }

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
