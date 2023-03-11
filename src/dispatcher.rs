use crate::events::*;
use crate::common::*;


pub struct Dispatcher {
    controllers: Vec<Box<dyn Controller>>,
    controller_idx: usize,
    screen: Screen,
}

pub const DefaultController: usize = 123456;

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
    pub fn dispatch(&mut self, controller_idx: usize, kind: EventKind, x: i32, y: i32) {
        let final_x = if x < 0 { 0 } else { x as usize };
        let final_y = if y < 0 { 0 } else { y as usize };
        self.controllers[if controller_idx == DefaultController { self.controller_idx } else { controller_idx }  ].dispatch(&self.screen, &kind, final_x, final_y);
    }
}