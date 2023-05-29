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
pub struct App {
    texture: Vec<u8>,
    dirty: bool,
    dispatcher: Option<Dispatcher>,
    models: Vec<Box<dyn Model>>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {

        let mut texture = Vec::new();
        for y in 0..=255 {
            for x in 0..=255 {
                texture.push(0);
                texture.push(0);
                texture.push(100);
                texture.push(255);
            }
        }


        let mut app = App {
            texture,
            dispatcher: None,
            dirty: true,
            models: Vec::new(),
        };
        app
    }
    pub fn add_fireworks_model(&mut self) {
        self.models.push(Box::new(create_fireworks_model(3000)));
    }
    pub fn add_drawing_editor_model(&mut self, width: usize, height: usize) {
        self.models.push(Box::new(drawing_editor::DrawingEditor::new(width, height, 3)));
    }
    pub fn add_extra_model(&mut self, width: usize, height: usize) {
        let mut hello_model = Box::new(drawing_editor::DrawingEditor::new(width, height, 1));
        hello_model.draw_rect(0, 0, 0, 100, 100, [1.0, 1.0, 0.0, 1.0]);
        self.models.push(hello_model);
    }
    pub fn init(&mut self, width: usize, height: usize, extra_model: bool) {
        let mut models: Vec<Box<dyn Model>> = self.models.drain(..).collect();

        let controllers: Vec<Box<dyn Controller>> = vec![
            Box::new(FireworksController::new()),
            Box::new(drawing_editor::DrawRectController::new()),
            Box::new(drawing_editor::DrawingEditorController::new()),
        ];
        let modelIndices = vec![0, 1, 1];

        for (model_idx, model) in models.iter().enumerate() {
            for (index, (pointer, length)) in model.buffers().into_iter().enumerate() {
                pass_buffer(model_idx, index, pointer, length);
            }
        }

        self.dispatcher = Some(Dispatcher::new(controllers, modelIndices, models, Screen {width, height}));
        return;
    }
    pub fn texture_pixels(&self) -> *const u8 {
        &self.texture[0]
    }
    pub fn update(&mut self) {
        self.dispatcher.as_mut().unwrap().update();
    }
    pub fn set_controller(&mut self,  controller_idx: usize) {
        self.dispatcher.as_mut().unwrap().set_controller(controller_idx);
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
        self.dispatcher.as_mut().unwrap().dispatch(controller_idx, kind, x, y);
        self.dirty = true;
    }
}
