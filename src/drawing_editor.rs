use wasm_bindgen::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::events::*;
use crate::common::*;

use std::cmp;

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
    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for x_ in x..x + width {
            for y_ in y..y + height {
                {
                    let idx = ((self.height - 1 - y_) * self.width + x_) * 4;
                    if idx + 3 > self.pixels.len() - 1 {
                        continue;
                    }
                    self.pixels[idx] = 255;
                    self.pixels[idx + 1] = 255;
                    self.pixels[idx + 2] = 255;
                    self.pixels[idx + 3] = 100;
    
                }
            }
        }
    }
}
pub struct DrawingEditorController {
    pointer_down: bool,
    model: Rc<RefCell<DrawingEditor>>,
}

impl DrawingEditorController {
    pub fn new(model: Rc<RefCell<DrawingEditor>>) -> DrawingEditorController {
        DrawingEditorController {pointer_down: false, model}
    }
    pub fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
        let thickness = 10;
        match kind {
            EventKind::PointerDown => {
                self.pointer_down = true;
                self.model.borrow_mut().draw_rect(x, y, thickness, thickness);
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    self.model.borrow_mut().draw_rect(x, y, thickness, thickness);
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
            }
            _ => ()
        }
    }
}

pub struct DrawRectController {
    model: Rc<RefCell<DrawingEditor>>,
    x0: usize,
    y0: usize,
}

impl DrawRectController {
    pub fn new(model: Rc<RefCell<DrawingEditor>>) -> DrawRectController {
        DrawRectController {model, x0: 0, y0: 0}
    }
    pub fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
        match kind {
            EventKind::PointerDown => {
                self.x0 = x;
                self.y0 = y;
            }
            EventKind::PointerUp => {
                let min_x = cmp::min(self.x0, x);
                let min_y = cmp::min(self.y0, y);
                let max_x = cmp::max(self.x0, x);
                let max_y = cmp::max(self.y0, y);

                self.model.borrow_mut().draw_rect(min_x, min_y, max_x - min_x, max_y - min_y);
            }
            _ => ()
        }
    }
}

