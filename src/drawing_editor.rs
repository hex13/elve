use wasm_bindgen::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::events::*;
use crate::common::*;

use std::cmp;

pub struct DrawingEditor {
    pub layers: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl DrawingEditor {
    pub fn new(width: usize, height: usize) -> DrawingEditor {
        let mut test_layer = vec![0; width * height * 4 /* rgba */ ];
        for (i, v) in test_layer.iter_mut().enumerate() {
            if i % 4 == 3 {
                *v = 50;
            } else if i % 4 == 1 {
                *v = 0;
            } else {
                *v = 0;
            }
        }
        DrawingEditor { 
            layers: vec![
                vec![0; width * height * 4 /* rgba */ ],
                vec![0; width * height * 4 /* rgba */ ],
                test_layer,
            ],
            width, height,
        }
    }
    pub fn pixels(&self, layer_idx: usize) -> *const u8 {
        &self.layers[layer_idx][0]
    }
    pub fn clear(&mut self, layer_idx: usize) {
        self.layers[layer_idx].fill(0);
    }
    pub fn draw_rect(&mut self, layer_idx: usize, x: usize, y: usize, width: usize, height: usize) {
        let mut layer = &mut self.layers[layer_idx];
        for x_ in x..x + width {
            for y_ in y..y + height {
                {
                    let idx = ((self.height - 1 - y_) * self.width + x_) * 4;
                    if idx + 3 > layer.len() - 1 {
                        continue;
                    }
                    layer[idx] = 255;
                    layer[idx + 1] = 255;
                    layer[idx + 2] = 255;
                    layer[idx + 3] = 100;
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
}

impl Controller for DrawingEditorController {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
        let thickness = 10;
        match kind {
            EventKind::PointerDown => {
                self.pointer_down = true;
                self.model.borrow_mut().draw_rect(0, x, y, thickness, thickness);
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    self.model.borrow_mut().draw_rect(0, x, y, thickness, thickness);
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
    pointer_down: bool,
    x0: usize,
    y0: usize,
}


impl Controller for DrawRectController {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
        let min_x = cmp::min(self.x0, x);
        let min_y = cmp::min(self.y0, y);
        let max_x = cmp::max(self.x0, x);
        let max_y = cmp::max(self.y0, y);

        match kind {
            EventKind::PointerDown => {
                self.pointer_down = true;
                self.x0 = x;
                self.y0 = y;
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    let mut model = self.model.borrow_mut();
                    model.clear(1);
                    model.draw_rect(1, min_x, min_y, max_x - min_x, max_y - min_y);
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
                self.model.borrow_mut().clear(1);
                self.model.borrow_mut().draw_rect(0, min_x, min_y, max_x - min_x, max_y - min_y);
            }
            _ => ()
        }
    }

}

impl DrawRectController {
    pub fn new(model: Rc<RefCell<DrawingEditor>>) -> DrawRectController {
        DrawRectController {model, x0: 0, y0: 0, pointer_down: false}
    }
}

