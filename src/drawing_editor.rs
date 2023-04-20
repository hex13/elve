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
                *v = 20;
            } else if i % 4 == 0 {
                *v = 255;
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
    pub fn draw_line(&mut self, layer_idx: usize, x0: usize, y0: usize, x1: usize, y1: usize, thickness: usize, color: Color) {
        let width = x1 as f32 - x0 as f32;
        let height = y1 as f32 - y0 as f32;
        let dist = ((width * width + height * height) as f32).sqrt();
        let dx = width / dist;
        let dy = height / dist;
        let mut x = x0 as f32;
        let mut y = y0 as f32;

        for step in 0..dist as usize {
            x += dx;
            y += dy;
            self.draw_rect(layer_idx, x as usize, y as usize, thickness, thickness, color);
        }
    }
    pub fn draw_rect(&mut self, layer_idx: usize, x: usize, y: usize, width: usize, height: usize, color: Color) {
        let mut layer = &mut self.layers[layer_idx];
        for x_ in x..x + width {
            for y_ in y..y + height {
                {
                    let idx = ((self.height - 1 - y_) * self.width + x_) * 4;
                    if idx + 3 > layer.len() - 1 {
                        continue;
                    }
                    layer[idx] = (color[0] * 255.0) as u8;
                    layer[idx + 1] = (color[1] * 255.0) as u8;
                    layer[idx + 2] = (color[2] * 255.0) as u8;
                    layer[idx + 3] = (color[3] * 255.0) as u8;
                }
            }
        }
    }
}
pub struct DrawingEditorController {
    pointer_down: bool,
    x0: usize,
    y0: usize,
    color: Color,
    model: Rc<RefCell<DrawingEditor>>,
}

impl DrawingEditorController {
    pub fn new(model: Rc<RefCell<DrawingEditor>>) -> DrawingEditorController {
        DrawingEditorController {pointer_down: false, model, x0: 0, y0: 0, color: [0.0, 0.0, 0.0, 1.0]}
    }
}

impl Controller for DrawingEditorController {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
        if let Some(action) = self.transform_input(screen, kind, x, y) {
            let thickness = 5;
            if let ActionPayload::DrawData(x0, y0, x1, y1, color) = action.payload {
                self.model.borrow_mut().draw_line(0, x0, y0, x1, y1, thickness, color);
            }
        }
    }
    fn transform_input(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) -> Option<Action> {
        let mut action = None;
        match kind {
            EventKind::PointerDown => {
                self.pointer_down = true;
                self.x0 = x;
                self.y0 = y;
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    action = Some(Action {kind: EventKind::Interact, x: 0.0, y: 0.0, payload: ActionPayload::DrawData(self.x0, self.y0, x, y, self.color)});
                    self.x0 = x;
                    self.y0 = y;
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
            }
            EventKind::ChangeColor => {
                let r = (x & 0xff0000) >> 16;
                let g = (x & 0xff00) >> 8;
                let b = x & 0xff;
                self.color = [(r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0, 1.0];
            }
            _ => ()
        }
        return action;
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
        let color = [1.0, 1.0, 1.0, 1.0];
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
                    model.draw_rect(1, min_x, min_y, max_x - min_x, max_y - min_y, color);
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
                self.model.borrow_mut().clear(1);
                self.model.borrow_mut().draw_rect(0, min_x, min_y, max_x - min_x, max_y - min_y, color);
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

