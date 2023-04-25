use crate::events::*;
use crate::common::*;


pub struct Dispatcher {
    controllers: Vec<Box<dyn Controller>>,
    pub controller_idx: usize,
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
    pub fn dispatch(&mut self, controller_idx: usize, kind: EventKind, x: i32, y: i32) -> (usize, Option<Action>) {
        let final_x = if x < 0 { 0 } else { x as usize };
        let final_y = if y < 0 { 0 } else { y as usize };


        let final_controller_idx = if controller_idx == DefaultController { self.controller_idx } else { controller_idx };
        let mut ctrl = &mut self.controllers[final_controller_idx];
        if let Some(action) = ctrl.transform_input(&self.screen, &kind, final_x, final_y) {
            (final_controller_idx, Some(action))
        }  else {
            ctrl.dispatch(&self.screen, &kind, final_x, final_y);
            (final_controller_idx, None)
        }
    }
}