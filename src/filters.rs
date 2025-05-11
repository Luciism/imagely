use std::f64::consts::E;
use std::f64::consts::PI;

use super::Image;

fn gaussian(x: isize, y: isize, sigma: f64) -> f64 {
    let exponent = -((x.pow(2) as f64 + y.pow(2) as f64) / (2.0 * sigma.powf(2.0)));
    (1.0 / (2.0 * PI * sigma.powf(2.0))) * E.powf(exponent)
}

struct Kernel;
impl Kernel {
    fn gaussian(size: usize, sigma: f64) -> Vec<f64> {
        let amplitude = (size as isize - 1) / 2;

        let mut kernel = Vec::with_capacity(size * size);

        for x in -amplitude..(amplitude + 1) {
            for y in -amplitude..(amplitude + 1) {
                kernel.push(gaussian(x, y, sigma));
            }
        }

        // Normalize kernel
        let kernel_sum: f64 = kernel.iter().sum();
        for i in 0..kernel.len() {
            kernel[i] = kernel[i] / kernel_sum;
        }

        kernel
    }
}

impl Image {
    // proof of concept
    pub fn mean_blur(&mut self) {
        let mut output_data: Vec<u8> = Vec::with_capacity(self.data.len());

        let width = self.width as usize;

        for i in 0..(self.data.len() / self.channels as usize) {
            let mut reds: Vec<u8> = Vec::new();
            let mut greens: Vec<u8> = Vec::new();
            let mut blues: Vec<u8> = Vec::new();

            if (i as isize - 1) / width as isize == (i / width) as isize && i as isize - 1 >= 0 {
                let left = i as usize - 1;
                reds.push(self.data[left * self.channels as usize]);
                greens.push(self.data[left * self.channels as usize + 1]);
                blues.push(self.data[left * self.channels as usize + 2]);
            }

            if (i + 1) / width == i / width && i + 1 < width * self.height as usize {
                let right = i + 1;
                reds.push(self.data[right * self.channels as usize]);
                greens.push(self.data[right * self.channels as usize + 1]);
                blues.push(self.data[right * self.channels as usize + 2]);
            }

            if i as isize - width as isize >= 0 {
                let top = i - width;
                reds.push(self.data[top * self.channels as usize]);
                greens.push(self.data[top * self.channels as usize + 1]);
                blues.push(self.data[top * self.channels as usize + 2]);
            }

            if i + width < width * self.height as usize {
                let bottom = i + width;
                reds.push(self.data[bottom * self.channels as usize]);
                greens.push(self.data[bottom * self.channels as usize + 1]);
                blues.push(self.data[bottom * self.channels as usize + 2]);
            }

            let red: u8 = (reds.iter().map(|&x| x as usize).sum::<usize>() / reds.len() as usize)
                .try_into()
                .unwrap();
            let green: u8 = (greens.iter().map(|&x| x as usize).sum::<usize>()
                / greens.len() as usize)
                .try_into()
                .unwrap();
            let blue: u8 = (blues.iter().map(|&x| x as usize).sum::<usize>()
                / blues.len() as usize)
                .try_into()
                .unwrap();

            output_data.push(red);
            output_data.push(green);
            output_data.push(blue);
            if self.channels == 4 {
                output_data.push(self.data[i * self.channels as usize + 3]);
            }
        }

        self.data = output_data;
    }

    pub fn gaussian_blur(&mut self, kernel_size: usize, sigma: f64) {
        // let kernel_size = 33;
        assert!(kernel_size % 2 == 1);

        let kernel = Kernel::gaussian(kernel_size, sigma);
        let kernel_amplitude = (kernel_size as isize - 1) / 2;

        let mut output_data: Vec<u8> = Vec::with_capacity(self.data.len());

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let mut rgb_sums = (0.0, 0.0, 0.0);
                let pixel_index = self.pos_to_index(x, y);

                // For each cell in the kernel
                for kernel_y in -kernel_amplitude..(kernel_amplitude + 1) {
                    for kernel_x in -kernel_amplitude..(kernel_amplitude + 1) {
                        // Ensure cell is within image bounds
                        let image_cell_x = x as isize + kernel_x;
                        let image_cell_y = y as isize + kernel_y;
                        if 0 <= image_cell_x
                            && image_cell_x < self.width as isize
                            && 0 <= image_cell_y
                            && image_cell_y < self.height as isize
                        {
                            let kernel_index = ((kernel_y + kernel_amplitude)
                                * kernel_size as isize
                                + (kernel_y + kernel_amplitude))
                                as usize;
                            let image_cell_index =
                                self.pos_to_index(image_cell_x as usize, image_cell_y as usize);
                            rgb_sums.0 += self.data[image_cell_index] as f64 * kernel[kernel_index];
                            rgb_sums.1 +=
                                self.data[image_cell_index + 1] as f64 * kernel[kernel_index];
                            rgb_sums.2 +=
                                self.data[image_cell_index + 2] as f64 * kernel[kernel_index];
                        }
                    }
                }

                output_data.push(rgb_sums.0 as u8);
                output_data.push(rgb_sums.1 as u8);
                output_data.push(rgb_sums.2 as u8);

                if self.channels == 4 {
                    output_data.push(self.data[pixel_index + 3]);
                }
            }
        }

        self.data = output_data;
    }
}
