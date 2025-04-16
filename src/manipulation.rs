use core::panic;

use super::Image;

impl Image {
    pub fn to_rgb(&mut self) {
        // Remove each alpha value
        let mut output_data: Vec<u8> = vec![];
        for (i, val) in self.data.iter().enumerate() {
            if (i + 1) % 4 != 0 {
                output_data.push(*val);
            }
        }
        self.data = output_data;
        self.channels = 3;
    }

    pub fn to_rgba(&mut self) {
        // Add in alpha after every rgb value.
        let mut output_data: Vec<u8> = vec![];
        for (i, val) in self.data.iter().enumerate() {
            output_data.push(*val);
            if (i + 1) % 3 == 0 {
                output_data.push(255);
            }
        }
        self.data = output_data;
        self.channels = 4;

    }

    pub fn make_compatible(&self, image2: &mut Image) {
        if self.channels == 4 && image2.channels != 4 {
            image2.to_rgba();
        }
        else if self.channels == 3 && image2.channels == 4 {
            image2.to_rgb();
        }
        else if self.channels != 4 && self.channels != 4 {
            panic!("[FATAL] Invalid number of channels: {}", image2.channels);
        }
    }
    
    pub fn paste(&mut self, mut image2: Image, position: (u32, u32)) {
        let starting_pos = position.1 * self.width + position.0;

        // TODO: RGB / RGBA conversion
        self.make_compatible(&mut image2);

        for i in 0..image2.height {
            let row_pos = starting_pos + self.width * i as u32;

            // Ensure position is within base image
            if i + position.1 >= self.height {
                break;
            }

            let splice_start = row_pos;
            let mut splice_end = row_pos + image2.width;

            // Prevent horizontal wraparound
            let max_x_pos = (i + position.1 + 1) * self.width;
            let mut cut_x = 0;

            if splice_end > max_x_pos {
                cut_x = splice_end - max_x_pos;
                splice_end = max_x_pos;
            }

            // Prevent vertical overflow
            let mut cut_y = 0;
            let data_len = self.data.len();
            if splice_end > data_len as u32 {
                cut_y = splice_end - data_len as u32;
                splice_end = data_len as u32;
            }

            let splice2_start = i * image2.width;
            let splice2_end = (i * image2.width + image2.width) - cut_x - cut_y;

            self.data.splice(
                splice_start as usize * self.channels as usize
                    ..
                splice_end as usize * self.channels as usize,
                image2.data[
                    splice2_start as usize * self.channels as usize
                    ..
                    splice2_end as usize * self.channels as usize
                ].to_vec(),
            );
        }
    }

    pub fn to_grayscale(&mut self) {
        for i in 0..self.data.len() / self.channels as usize {
            let pos = i * self.channels as usize;
            let avg_rgb = (self.data[pos] as u16 + self.data[pos + 1] as u16 + self.data[pos + 2] as u16) / 3;

            self.data[pos] = avg_rgb as u8;
            self.data[pos + 1] = avg_rgb as u8;
            self.data[pos + 2] = avg_rgb as u8;
        }
    }


 
    fn rotate_90_helper(&mut self, calc: fn(usize, &mut Image) -> usize) {
        let mut output_data: Vec<u8> = Vec::with_capacity(self.data.len());
        output_data.resize(self.data.len(), 0);

        for i in 0..self.data.len() / self.channels as usize {
            let pos = i * self.channels as usize;

            let new_pixel_index = calc(i, self);

            output_data[new_pixel_index] = self.data[pos];
            output_data[new_pixel_index + 1] = self.data[pos + 1];
            output_data[new_pixel_index + 2] = self.data[pos + 2];

            if self.channels == 4 {
                output_data[new_pixel_index + 3] = self.data[pos + 3];
            }
        }
        self.data = output_data;

        // Swap width and height values
        let height = self.width;
        self.width = self.height;
        self.height = height;
    }

    pub fn rotate_90(&mut self) {
        self.rotate_90_helper(|i, img| {
            let row = (img.height - 1) - i as u32 / (img.width);
            let new_pixel_index = i as u32 % img.width * img.height + row;
            new_pixel_index as usize * img.channels as usize
        });
    }  

    pub fn rotate_270(&mut self) {
        self.rotate_90_helper(|i, img| {
            let row = img.height - i as u32 / img.width;
            let new_pixel_index = img.width * img.height - (i as u32 % img.width * img.height + row);
            new_pixel_index as usize * img.channels as usize
        });
    }

    pub fn rotate_180(&mut self) {
        let mut output_data: Vec<u8> = Vec::with_capacity(self.data.len());

        for i in (0..self.data.len() / self.channels as usize).rev() {
            let pos = i * self.channels as usize;

            output_data.push(self.data[pos]);
            output_data.push(self.data[pos + 1]);
            output_data.push(self.data[pos + 2]);

            if self.channels == 4 {
                // output_data[(new_pixel_index + 3) as usize] = self.data[pos + 3];
            output_data.push(self.data[pos + 3]);
            }
        }
        self.data = output_data;
    }


    pub fn crop(&mut self, rect: (u32, u32, u32, u32)) {
        assert!(rect.2 >= rect.0);
        assert!(rect.3 >= rect.1);
        assert!(rect.0 <= self.width && rect.2 <= self.width);
        assert!(rect.1 <= self.height && rect.3 <= self.height);

        let crop_width = rect.2 - rect.0;
        let crop_height = rect.3 - rect.1;

        let mut output_data: Vec<u8> = vec![];
        let start_pos = rect.1 * self.width + rect.0; // Translate (x, y) into 1d

        for i in 0..crop_height {
            let slice = &self.data[
                ((start_pos + (self.width * i)) * self.channels as u32) as usize..
                ((start_pos + (self.width * i) + crop_width) * self.channels as u32) as usize
            ];
            output_data.extend_from_slice(slice);
        };

        self.width = crop_width as u32;
        self.height = crop_height as u32;
        self.data = output_data;
    }
}
