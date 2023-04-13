use wasm_bindgen::prelude::*;
mod particles;
mod drawing_editor;
use drawing_editor::*;
use std::rc::Rc;
use std::cell::RefCell;
mod events;
use events::*;
mod common;
use common::*;
mod tilemap;

mod dispatcher;
use dispatcher::*;




impl Controller for FireworksController {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
        let ndc_x = (x as f32 / screen.width as f32) * 2.0 - 1.0;
        let ndc_y = -((y as f32 / screen.height as f32) * 2.0 - 1.0);
        match kind {
            EventKind::PointerDown => {
                self.model.borrow_mut().create_explosion_at(ndc_x, ndc_y);
                self.pointer_down = true;
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    self.model.borrow_mut().create_explosion_at(ndc_x, ndc_y);
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
            }
            EventKind::TogglePlay => {
                self.model.borrow_mut().togglePlay();
            }
            _ => ()
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=window)]
    pub fn pass_buffer(index: usize, pointer: *const f32);
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: String);
}

pub struct FireworksController {
    pointer_down: bool,
    model: Rc<RefCell<ParticleSystemModel>>,
}

impl FireworksController {
    pub fn new(model: Rc<RefCell<ParticleSystemModel>>) -> FireworksController {
        FireworksController {pointer_down: false, model}
    }
}





pub enum Resource {
    ParticleSystem(particles::ParticleSystem),
}


pub struct ParticleSystemModel {
    particle_system_state: particles::ParticleSystemState,
}

impl Model for ParticleSystemModel {
    fn buffers(&self) -> Vec<(*const f32)>{
        return vec![&self.particle_system_state.positions[0], &self.particle_system_state.colors[0]];
    }
}


impl ParticleSystemModel {
    pub fn update(&mut self) {
        particles::ParticleSystem::update(&mut self.particle_system_state)
    }
    pub fn togglePlay(&mut self) {
        self.particle_system_state.autoexplosions = !self.particle_system_state.autoexplosions;
    }
    pub fn create_explosion_at(&mut self, x: f32, y: f32) {
        particles::ParticleSystem::create_explosion_at(&mut self.particle_system_state, x, y)
    }
}



pub fn create_fireworks_model(count: usize) -> ParticleSystemModel {

    let unit = 0.2;
    let positions = vec![0.0; count * 2];
    let velocities = vec![0.0; count * 2];
    let colors = vec![0.0; count * 4];
    let explosions = Vec::new();
    let particle_system_state = particles::ParticleSystemState { count, positions, velocities, colors, explosions, autoexplosions: false };

    ParticleSystemModel {
        particle_system_state,
    }
}






#[wasm_bindgen]
struct App {
    fireworks: Rc<RefCell<ParticleSystemModel>>,
    drawing_editor: Rc<RefCell<DrawingEditor>>,
    texture: Vec<u8>,
    dirty: bool,
    dispatcher: Dispatcher,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> App {
        let drawing_editor = Rc::new(RefCell::new(drawing_editor::DrawingEditor::new(width, height)));
        let fireworks = Rc::new(RefCell::new(create_fireworks_model(3000)));
        for (index, pointer) in fireworks.borrow().buffers().into_iter().enumerate() {
            pass_buffer(index, pointer);
        }

        let mut texture = Vec::new();
        for y in 0..=255 {
            for x in 0..=255 {
                texture.push(0);
                texture.push(0);
                texture.push(100);
                texture.push(255);
            }
        }
        App {
            fireworks: Rc::clone(&fireworks),
            drawing_editor: Rc::clone(&drawing_editor),
            texture,
            dispatcher: Dispatcher::new(vec![
                Box::new(FireworksController::new(Rc::clone(&fireworks))),
                Box::new(drawing_editor::DrawRectController::new(Rc::clone(&drawing_editor))),
                Box::new(drawing_editor::DrawingEditorController::new(Rc::clone(&drawing_editor))),
            ], Screen {width, height}),
            dirty: true,
        }
    }
    pub fn texture_pixels(&self) -> *const u8 {
        &self.texture[0]
    }
    pub fn drawing_editor_pixels(&self, layer_idx: usize) -> *const u8 {
        self.drawing_editor.borrow().pixels(layer_idx)
    }
    pub fn update(&mut self) {
        self.fireworks.borrow_mut().update();
    }
    pub fn set_controller(&mut self,  controller_idx: usize) {
        self.dispatcher.set_controller(controller_idx);
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn set_dirty(&mut self, value: bool) {
        self.dirty = value;
    }
    pub fn dispatch(&mut self, kind: EventKind, x: i32, y: i32) {
        self.dispatch_to(DefaultController, kind, x, y);
    }
    pub fn dispatch_to(&mut self, controller_idx: usize, kind: EventKind, x: i32, y: i32) {
        self.dispatcher.dispatch(controller_idx, kind, x, y);
        self.dirty = true;
    }
}
