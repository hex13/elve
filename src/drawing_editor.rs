use wasm_bindgen::prelude::*;

pub struct DrawingEditor {
    pub pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl DrawingEditor {
    pub fn new(width: usize, height: usize) -> DrawingEditor {
        DrawingEditor { 
            pixels: vec![0; width * height * 4 /* rgba */ ],
            width, height,
        }
    }
    pub fn pixels(&self) -> *const u8 {
        &self.pixels[0]
    }
    pub fn draw(&mut self, x: usize, y: usize) {
        for x_ in x..x + 4 {
            for y_ in y..y + 4 {
                {
                    let idx = ((self.height - 1 - y_) * self.width + x_) * 4;
                    self.pixels[idx] = 255;
                    self.pixels[idx + 1] = 255;
                    self.pixels[idx + 2] = 255;
                    self.pixels[idx + 3] = 100;
    
                }
            }
        }
    }
}