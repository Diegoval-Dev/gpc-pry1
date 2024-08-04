use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0); // Default background color: black
        let current_color = Color::new(255, 255, 255); // Default current color: white
        let buffer = vec![background_color.to_hex(); width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color.to_hex());
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y as usize * self.width + x as usize;
            self.buffer[index] = self.current_color.to_hex();
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}
