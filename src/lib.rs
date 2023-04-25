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


pub struct FireworksController {
    pointer_down: bool,
}

impl FireworksController {
    pub fn new() -> FireworksController {
        FireworksController {pointer_down: false}
    }
}

impl Controller for FireworksController {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
    }
    fn transform_input(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) -> Option<Action> {
        let ndc_x = (x as f32 / screen.width as f32) * 2.0 - 1.0;
        let ndc_y = -((y as f32 / screen.height as f32) * 2.0 - 1.0);

        match kind {
            EventKind::PointerDown => {
                self.pointer_down = true;
                Some(Action {kind: EventKind::Interact, x: ndc_x, y: ndc_y, payload: ActionPayload::Empty})
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    Some(Action {kind: EventKind::Interact, x: ndc_x, y: ndc_y, payload: ActionPayload::Empty})
                } else {
                    None
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
                None
            }
            EventKind::TogglePlay => {
                Some(Action {kind: EventKind::TogglePlay, x: ndc_x, y: ndc_y, payload: ActionPayload::Empty})
            }
            _ => None
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=window)]
    pub fn pass_buffer(model_index: usize, index: usize, pointer: *const f32, length: usize);
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: String);
}




pub enum Resource {
    ParticleSystem(particles::ParticleSystem),
}


pub struct ParticleSystemModel {
    particle_system_state: RefCell<particles::ParticleSystemState>,
}

impl Model for ParticleSystemModel {
    fn buffers(&self) -> Vec<(*const f32, usize)>{
        let positions = &self.particle_system_state.borrow().positions;
        let colors = &self.particle_system_state.borrow().colors;
        return vec![
            (&positions[0], positions.len()),
            (&colors[0], colors.len()),
        ];
    }
    fn update(&self) {
        particles::ParticleSystem::update(&mut self.particle_system_state.borrow_mut());
    }
    fn act(&mut self, action: &Action) {
        match action.kind {
            EventKind::TogglePlay => {
                let mut state = self.particle_system_state.borrow_mut();
                state.autoexplosions = !state.autoexplosions;
            }
            EventKind::Interact => {
                particles::ParticleSystem::create_explosion_at(&mut self.particle_system_state.borrow_mut(), action.x, action.y);
            }
            _ => {}
        }
    }
}


pub fn create_fireworks_model(count: usize) -> ParticleSystemModel {

    let unit = 0.2;
    let positions = vec![0.0; count * 2];
    let velocities = vec![0.0; count * 2];
    let colors = vec![0.0; count * 4];
    let explosions = Vec::new();
    let particle_system_state = RefCell::new(particles::ParticleSystemState { count, positions, velocities, colors, explosions, autoexplosions: false });

    ParticleSystemModel {
        particle_system_state,
    }
}






#[wasm_bindgen]
struct App {
    models: Vec<Box<dyn Model>>,
    texture: Vec<u8>,
    dirty: bool,
    dispatcher: Dispatcher,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> App {
        let drawing_editor = Box::new(drawing_editor::DrawingEditor::new(width, height));
        let fireworks = Box::new(create_fireworks_model(3000));
        let models: Vec<Box<dyn Model>> = vec![fireworks, drawing_editor];

        for (model_idx, model) in models.iter().enumerate() {
            for (index, (pointer, length)) in model.buffers().into_iter().enumerate() {
                pass_buffer(model_idx, index, pointer, length);
            }
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
            texture,
            dispatcher: Dispatcher::new(vec![
                Box::new(FireworksController::new()),
                Box::new(drawing_editor::DrawRectController::new()),
                Box::new(drawing_editor::DrawingEditorController::new()),
            ], Screen {width, height}),
            dirty: true,
            models,
        }
    }
    pub fn texture_pixels(&self) -> *const u8 {
        &self.texture[0]
    }
    pub fn update(&mut self) {
        for model in &self.models {
            model.update();
        }
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
        if let (ctrl_idx, Some(action)) = self.dispatcher.dispatch(controller_idx, kind, x, y) {
            let model_idx = match ctrl_idx {
                0 => 0,
                1 => 1,
                2 => 1,
                _ => panic!("incorrect value for ctrl_idx"),
            };
            self.models[model_idx].act(&action);
        }
        self.dirty = true;
    }
}
