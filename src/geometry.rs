use super::Image;


struct Circle {
    center_pos: (usize, usize),
    radius: usize, // Does not include center pixel

}

impl Circle {
    fn build(center_pos: (usize, usize), radius: usize) -> Circle {
        Circle {center_pos, radius}
    }

    fn circle_pos(&self, edge_pos: (isize, isize)) -> (isize, isize) {
        let (edge_x, edge_y) = edge_pos;

        let y_displacement = edge_y as f64 - self.center_pos.1 as f64;
        let x_displacement = edge_x as f64 - self.center_pos.0 as f64;
        let theta = (y_displacement.abs() / x_displacement.abs()).atan();

        let mut x = (self.radius as f64 * theta.cos()).round() as isize;
        let mut y = (self.radius as f64 * theta.sin()).round() as isize;

        // Ensure correct positive / negative value
        x *= (x_displacement / x_displacement.abs()) as isize;
        y *= (y_displacement / y_displacement.abs()) as isize;

        (x + self.center_pos.0 as isize, y + self.center_pos.1 as isize)
    }


    // Function name could use some work
    fn bounding_pos_to_pos(&self, pos: (usize, usize)) -> (isize, isize) {
        let abs_pos = Image::sum_pos(pos, self.center_pos);
        Image::subtract_pos(abs_pos, (self.radius, self.radius))
    }

    pub fn draw_outline(&self, image: &mut Image, color: (u8, u8, u8, u8)) {
        let width = self.radius * 2 + 1;
        let color = vec![color.0, color.1, color.2, color.3];

        // Find circle pos corresponding to each circle bounding box pixel
        for i in 0..width {
            let top_pos = self.bounding_pos_to_pos((i, 0));
            let bottom_pos = self.bounding_pos_to_pos((i, width - 1));
            let left_pos = self.bounding_pos_to_pos((0, i));
            let right_pos = self.bounding_pos_to_pos((width - 1, i));

            image.replace_pixel_if_viable(self.circle_pos(top_pos), &color);
            image.replace_pixel_if_viable(self.circle_pos(bottom_pos), &color);
            image.replace_pixel_if_viable(self.circle_pos(left_pos), &color);
            image.replace_pixel_if_viable(self.circle_pos(right_pos), &color);
        }
    }
}

impl Image {
    fn replace_pixel_at(&mut self, pos: (u32, u32), value: &Vec<u8>) {
        let index = self.pos_to_index(pos.0 as usize, pos.1 as usize);

        self.data[index] = value[0];
        self.data[index + 1] = value[1];
        self.data[index + 2] = value[2];

        if self.channels == 4 {
            self.data[index + 3] = value[3];
        }
    }

    pub fn replace_pixel_if_viable(&mut self, pos: (isize, isize), color: &Vec<u8>) {
        if self.is_pos_in_image(pos) {
            self.replace_pixel_at((pos.0 as u32, pos.1 as u32), color);
        }
    }

    pub fn draw_circle_outline(&mut self, center_pos: (usize, usize), radius: usize, color: (u8, u8, u8, u8)) {
        Circle::build(center_pos, radius).draw_outline(self, color);
    }
}
