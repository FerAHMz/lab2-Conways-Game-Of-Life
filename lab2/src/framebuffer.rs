use raylib::prelude::*;
use std::convert::TryInto;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            background_color,
        );
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
            self.background_color,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            let data = self.color_buffer.data as *mut u8;
            unsafe {
                Color::new(
                    *data.add(index),
                    *data.add(index + 1),
                    *data.add(index + 2),
                    *data.add(index + 3),
                )
            }
        } else {
            self.background_color
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
}
