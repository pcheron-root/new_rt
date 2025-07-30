use std::fs::File;
use std::io::{self, Write};

use crate::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        if width == 0 || height == 0 {
            panic!("Cannot create a canvas with zero or negative size");
        }

        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        *self.at_mut(x, y) = color;
    }

    pub fn get_mut_pixel(&mut self, x: usize, y: usize) -> &mut Color {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            &mut self.pixels[index]
        } else {
            panic!("Error: can't get pixel at x:{} y:{}", x, y);
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index].clone()
        } else {
            panic!("Error: can't get pixel at x:{} y:{}", x, y);
        }
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        let content = self.to_ppm();
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
