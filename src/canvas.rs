use crate::util::Point;
use serde::Deserialize;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufColor(pub u32);

impl From<RgbColor> for BufColor {
    fn from(rgb: RgbColor) -> BufColor {
        let (r, g, b) = (rgb.0[0] as u32, rgb.0[1] as u32, rgb.0[2] as u32);
        BufColor((r << 16) | (g << 8) | b)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Debug)]
pub struct RgbColor(pub [u8; 3]);

impl From<BufColor> for RgbColor {
    fn from(color: BufColor) -> RgbColor {
        let [_, r, g, b] = color.0.to_be_bytes();
        RgbColor([r, g, b])
    }
}

#[derive(Clone)]
pub struct CanvasBuffer {
    data: Vec<BufColor>,
    width: u32,
    height: u32,
}

impl CanvasBuffer {
    pub fn new(data: Vec<BufColor>, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &[BufColor] {
        &self.data
    }

    pub unsafe fn raw_data(&self) -> &[u32] {
        std::slice::from_raw_parts(self.data.as_ptr() as *const u32, self.data.len())
    }

    fn index(&self, x: u32, y: u32) -> usize {
        y as usize * self.width as usize + x as usize
    }

    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: BufColor) {
        let index = self.index(x, y);
        self.data[index] = color;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> BufColor {
        let index = self.index(x, y);
        self.data[index]
    }

    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut BufColor {
        let index = self.index(x, y);
        &mut self.data[index]
    }

    /// Crops the image. Returns true if the crop was succesful.
    pub fn crop(&mut self, x: u32, y: u32, w: u32, h: u32) -> bool {
        let mut new_buf = vec![BufColor(0); w as usize * h as usize];

        if !self.in_bounds(x, y) || !self.in_bounds(x + w, y + h) {
            return false;
        }

        for i in 0..w {
            for j in 0..h {
                let old_index = self.index(x + i, y + j);
                let new_index = (j as usize * w as usize) + i as usize;

                new_buf[new_index] = self.data[old_index];
            }
        }
        self.data = new_buf;
        self.width = w;
        self.height = h;
        true
    }
}

pub enum ColorbarPos {
    Top,
    Bottom,
}

// Unsigned draw functions (u32)
impl CanvasBuffer {
    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: BufColor) {
        let border_x = self.width;
        let border_y = self.height;

        let x_min = x.clamp(0, border_x);
        let x_max = (x + w).clamp(0, border_x);

        let y_min = y.clamp(0, border_y);
        let y_max = (y + h).clamp(0, border_y);

        for x in x_min..x_max {
            for y in y_min..y_max {
                self.put_pixel(x, y, color);
            }
        }
    }

}

// Signed draw functions (i32)
impl CanvasBuffer {
    pub fn draw_square_s(&mut self, x: i32, y: i32, size: i32, color: BufColor) {
        let half = (size as f64 / 2.0) as i32;
        self.draw_rect_s(x - half, y - half, size, size, color);
    }

    pub fn draw_rect_s(&mut self, x: i32, y: i32, w: i32, h: i32, color: BufColor) {
        let border_x = self.width as i32;
        let border_y = self.height as i32;

        let x_min = x.clamp(0, border_x) as u32;
        let x_max = (x + w).clamp(0, border_x) as u32;

        let y_min = y.clamp(0, border_y) as u32;
        let y_max = (y + h).clamp(0, border_y) as u32;

        for x in x_min..x_max {
            for y in y_min..y_max {
                self.put_pixel(x, y, color);
            }
        }
    }
}

// UI draw functions
impl CanvasBuffer {
    pub fn draw_colorbar(&mut self, colors: &[RgbColor], selected: u32, pos: ColorbarPos) {
        let count = colors.len() as u32;
        let box_size = 32.min((self.width / count).max(3));
        let padding = 2.min((self.width as i32 - (box_size * count) as i32).max(0) as u32 / count);
        let margin = 2.min(padding);

        // Border width
        let bw = 2.min(box_size / 3);

        let y = match pos {
            ColorbarPos::Top => margin,
            ColorbarPos::Bottom => self.height - margin - box_size,
        };

        for (i, color) in colors
            .iter()
            .map(|c| BufColor::from(*c))
            .enumerate()
            .map(|(i, c)| (i as u32, c))
        {
            let x = margin + (box_size + padding) * i;
            // border color
            let bc = if i == selected {
                // Invert the color
                BufColor(0x00FFFFFF - color.0)
            } else {
                BufColor(0)
            };

            self.draw_rect(x, y, box_size, box_size, bc);
            self.draw_rect(x + bw, y + bw, box_size - 2 * bw, box_size - 2 * bw, color);
        }
    }

    pub fn draw_guides(&mut self, a: Point, b: Point) {
        let (bw, bh) = (self.width() as i32, self.height() as i32);
        let mut invert_pixel = move |x, y| {
            let pix = self.get_pixel_mut(x as u32, y as u32);
            let inverted = BufColor(0x00FFFFFF - pix.0);

            self.put_pixel(x as u32, y as u32, inverted);
        };

        if a.1 > 0 && a.1 < bh {
            for x in 0..bw {
                invert_pixel(x, a.1);
            }
        }

        if b.1 > 0 && b.1 < bh && a.1 != b.1 {
            for x in 0..bw {
                invert_pixel(x, b.1);
            }
        }

        if a.0 > 0 && a.0 < bw {
            for y in 0..bh {
                invert_pixel(a.0, y);
            }
        }

        if b.0 > 0 && b.0 < bw && a.0 != b.0 {
            for y in 0..bh {
                invert_pixel(b.0, y);
            }
        }
    }
}
