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
        for (x, y) in LineIterator::new(x0, y0, x1, y1) {
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

impl Model for DrawingEditor {
    fn buffers(&self) -> Vec<(*const f32, usize)> {
        self.layers.iter().map(|layer| {
            ((&layer[0] as *const u8) as *const f32, layer.len())
        }).collect()
    }
    fn update(&self) {

    }
    fn act(&mut self, action: &Action) {
        match action.kind {
            EventKind::DrawLine => {
                let thickness = 5;
                if let ActionPayload::DrawData(layer, x0, y0, x1, y1, color) = action.payload {
                    self.draw_line(layer, x0, y0, x1, y1, thickness, color);
                }
            }
            EventKind::DrawRectangle => {
                self.clear(1);
                if let ActionPayload::DrawData(layer, x0, y0, x1, y1, color) = action.payload {
                    self.draw_rect(layer, x0, y0, x1, y1, color);
                }
            },
            _ => {}
        }
    }
}

pub struct DrawingEditorController {
    pointer_down: bool,
    x0: usize,
    y0: usize,
    color: Color,
}

impl DrawingEditorController {
    pub fn new() -> DrawingEditorController {
        DrawingEditorController {pointer_down: false, x0: 0, y0: 0, color: [0.0, 0.0, 0.0, 1.0]}
    }
}

impl Controller for DrawingEditorController {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
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
                    action = Some(Action {kind: EventKind::DrawLine, x: 0.0, y: 0.0, payload: ActionPayload::DrawData(0, self.x0, self.y0, x, y, self.color)});
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
    pointer_down: bool,
    x0: usize,
    y0: usize,
}


impl Controller for DrawRectController {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
    }
    fn transform_input(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) -> Option<Action> {
        let min_x = cmp::min(self.x0, x);
        let min_y = cmp::min(self.y0, y);
        let max_x = cmp::max(self.x0, x);
        let max_y = cmp::max(self.y0, y);
        let color = [1.0, 1.0, 1.0, 1.0];
        let mut action = None;
        match kind {
            EventKind::PointerDown => {
                self.pointer_down = true;
                self.x0 = x;
                self.y0 = y;
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    action = Some(Action { kind: EventKind::DrawRectangle, x: 0.0, y: 0.0, payload: ActionPayload::DrawData(1, min_x, min_y, max_x - min_x, max_y - min_y, color)});
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
                action = Some(Action { kind: EventKind::DrawRectangle, x: 0.0, y: 0.0, payload: ActionPayload::DrawData(0, min_x, min_y, max_x - min_x, max_y - min_y, color)});
            }
            _ => ()
        }
        return action;
    }

}

impl DrawRectController {
    pub fn new() -> DrawRectController {
        DrawRectController {x0: 0, y0: 0, pointer_down: false}
    }
}


struct LineIterator {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    x: f32,
    y: f32,
    alpha: f32,
}

impl LineIterator {
    fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> LineIterator {
        LineIterator { x0, y0, x1, y1, x: x0 as f32, y: y0 as f32, alpha: 0.0}
    }
}

impl Iterator for LineIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let width = (self.x1 as f32 - self.x0 as f32).abs();
        let height = (self.y1 as f32 - self.y0 as f32).abs();
        let dist = ((width * width + height * height) as f32).sqrt();
        let dir_x = if self.x1 >= self.x0 { 1.0 } else { -1.0 };
        let dir_y = if self.y1 >= self.y0 { 1.0 } else { -1.0 };
        let current = (
            ((self.x0 as f32) + dir_x * width * self.alpha) as usize,
            ((self.y0 as f32) + dir_y * height  * self.alpha) as usize,
        );
        let steps = if width > height { width } else { height };
        if self.alpha > 1.0 {
            return None;
        }
        self.alpha += 1.0 / steps;
        return Some(current);
    }
}


mod tests {
    use super::*;
    #[test]
    fn line_iterator_horizontal() {
        let v: Vec<(usize, usize)> = LineIterator::new(2, 2, 5, 2).collect();
        assert_eq!(v[0], (2, 2));
        assert_eq!(v[1], (3, 2));
        assert_eq!(v[2], (4, 2));
        assert_eq!(v[3], (5, 2));
        assert_eq!(v.len(), 4);
    }
    #[test]
    fn line_iterator_horizontal_reverse() {
        let v: Vec<(usize, usize)> = LineIterator::new(5, 2, 2, 2).collect();
        println!("{:?}", v);
        assert_eq!(v[0], (5, 2));
        assert_eq!(v[1], (4, 2));
        assert_eq!(v[2], (3, 2));
        assert_eq!(v[3], (2, 2));
        assert_eq!(v.len(), 4);
    }
    #[test]
    fn line_iterator_vertical() {
        let v: Vec<(usize, usize)> = LineIterator::new(3, 2, 3, 7).collect();
        assert_eq!(v[0], (3, 2));
        assert_eq!(v[1], (3, 3));
        assert_eq!(v[2], (3, 4));
        assert_eq!(v[3], (3, 5));
        assert_eq!(v[4], (3, 6));
        assert_eq!(v[5], (3, 7));
        assert_eq!(v.len(), 6);
    }
    #[test]
    fn line_iterator_diagonal() {
        let v: Vec<(usize, usize)> = LineIterator::new(1, 0, 5, 4).collect();
        println!("{:?}", v);
        assert_eq!(v[0], (1, 0));
        assert_eq!(v[1], (2, 1));
        assert_eq!(v[2], (3, 2));
        assert_eq!(v[3], (4, 3));
        assert_eq!(v[4], (5, 4));
        assert_eq!(v.len(), 5);
    }
    #[test]
    fn line_iterator_diagonal_reverse() {
        let v: Vec<(usize, usize)> = LineIterator::new(5, 4, 1, 0).collect();
        println!("{:?}", v);
        assert_eq!(v[0], (5, 4));
        assert_eq!(v[1], (4, 3));
        assert_eq!(v[2], (3, 2));
        assert_eq!(v[3], (2, 1));
        assert_eq!(v[4], (1, 0));
        assert_eq!(v.len(), 5);
    }
}