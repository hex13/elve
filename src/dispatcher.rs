use crate::events::*;
use crate::common::*;


pub struct Dispatcher {
    controllers: Vec<Box<dyn Controller>>,
    controller_idx: usize,
    screen: Screen,
}

impl Dispatcher {
    pub fn new(controllers: Vec<Box<dyn Controller>>, screen: Screen) -> Dispatcher {
        Dispatcher {
            controllers,
            controller_idx: 0,
            screen,
        }
    }
    pub fn set_controller(&mut self,  controller_idx: usize) {
        self.controller_idx = controller_idx;
    }
    pub fn dispatch(&mut self, kind: EventKind, x: i32, y: i32) {
        self.dispatch_to(self.controller_idx, kind, x, y);
    }
    pub fn dispatch_to(&mut self, controller_idx: usize, kind: EventKind, x: i32, y: i32) {
        let final_x = if x < 0 { 0 } else { x as usize };
        let final_y = if y < 0 { 0 } else { y as usize };
        self.controllers[controller_idx].dispatch(&self.screen, &kind, final_x, final_y);
    }
}