use crate::events::*;
use crate::common::*;

type Listener = Box<dyn FnMut()>;

pub struct Dispatcher {
    controllers: Vec<Box<dyn Controller>>,
    models: Vec<Box<dyn Model>>,
    modelIndices: Vec<usize>,
    pub controller_idx: usize,
    screen: Screen,
    once_listeners: Vec<(EventKind, Listener)>
}

pub const DefaultController: usize = 123456;

impl Dispatcher {
    pub fn new(controllers: Vec<Box<dyn Controller>>, modelIndices: Vec<usize>, models: Vec<Box<dyn Model>>, screen: Screen) -> Dispatcher {
        Dispatcher {
            models,
            modelIndices,
            controllers,
            controller_idx: 0,
            screen,
            once_listeners: Vec::new(),
        }
    }
    pub fn once(&mut self, kind: EventKind, handler: Listener) {
        self.once_listeners.push((kind, handler));
    }
    pub fn set_controller(&mut self,  controller_idx: usize) {
        self.controller_idx = controller_idx;
    }
    pub fn update(&mut self) {
        for model in &self.models {
            model.update();
        }
    }
    pub fn dispatch(&mut self, controller_idx: usize, kind: EventKind, x: i32, y: i32) {
        let final_x = if x < 0 { 0 } else { x as usize };
        let final_y = if y < 0 { 0 } else { y as usize };

        let final_controller_idx = if controller_idx == DefaultController { self.controller_idx } else { controller_idx };
        let mut model = &mut self.models[self.modelIndices[final_controller_idx]];
        let mut ctrl = &mut self.controllers[final_controller_idx];
        if let Some(action) = ctrl.transform_input(&self.screen, &kind, final_x, final_y) {
            model.act(&action);
        }  else {
            ctrl.dispatch(&self.screen, &kind, final_x, final_y);
        }
        for (listener_kind, listener) in self.once_listeners.iter_mut() {
            if *listener_kind == kind {
                listener();
            }
        }
        self.once_listeners.retain(|(listener_kind, _)| { *listener_kind != kind });
    }
}