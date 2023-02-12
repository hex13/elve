use wasm_bindgen::prelude::*;
mod particles;
mod drawing_editor;
use drawing_editor::*;
use std::rc::Rc;
use std::cell::RefCell;
mod events;
use events::*;




#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=window)]
    pub fn pass_firework_buffers(positions: *const f32, colors: *const f32 );
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: String);
}

#[wasm_bindgen]
pub struct FireworksController {
    pointer_down: bool,
    model: Rc<RefCell<ParticleSystemModel>>,
}

impl FireworksController {
    pub fn new(model: Rc<RefCell<ParticleSystemModel>>) -> FireworksController {
        FireworksController {pointer_down: false, model}
    }
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



pub struct DrawingEditorController {
    pointer_down: bool,
    model: Rc<RefCell<DrawingEditor>>,
}

impl DrawingEditorController {
    pub fn new(model: Rc<RefCell<DrawingEditor>>) -> DrawingEditorController {
        DrawingEditorController {pointer_down: false, model}
    }
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) {
        match kind {
            EventKind::PointerDown => {
                self.pointer_down = true;
                self.model.borrow_mut().draw(x, y);
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    self.model.borrow_mut().draw(x, y);
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
            }
            _ => ()
        }
    }
}


pub enum Resource {
    ParticleSystem(particles::ParticleSystem),
}


#[wasm_bindgen]
pub struct ParticleSystemModel {
    particle_system_state: particles::ParticleSystemState,
}

#[wasm_bindgen]
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


#[wasm_bindgen]
pub fn create_fireworks_model(count: usize) -> ParticleSystemModel {

    let unit = 0.2;
    let positions = vec![0.0; count * 2];
    let velocities = vec![0.0; count * 2];
    let colors = vec![0.0; count * 4];
    let explosions = Vec::new();
    let particle_system_state = particles::ParticleSystemState { count, positions, velocities, colors, explosions, autoexplosions: false };

    pass_firework_buffers(&particle_system_state.positions[0], &particle_system_state.colors[0]);
    ParticleSystemModel {
        particle_system_state,
    }
}


struct Screen {
    width: usize,
    height: usize,
}




#[wasm_bindgen]
struct App {
    fireworks: Rc<RefCell<ParticleSystemModel>>,
    drawing_editor: Rc<RefCell<DrawingEditor>>,
    controller: FireworksController,
    drawing_editor_controller: DrawingEditorController,
    screen: Screen,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> App {
        let drawing_editor = Rc::new(RefCell::new(drawing_editor::DrawingEditor::new(width, height)));
        let fireworks = Rc::new(RefCell::new(create_fireworks_model(3000)));
        App {
            fireworks: Rc::clone(&fireworks),
            controller: FireworksController::new(Rc::clone(&fireworks)),
            drawing_editor: Rc::clone(&drawing_editor),
            screen: Screen {width, height},
            drawing_editor_controller: DrawingEditorController::new(Rc::clone(&drawing_editor)),
        }
    }
    pub fn drawing_editor_pixels(&self) -> *const u8 {
        &self.drawing_editor.borrow().pixels[0]
    }
    pub fn update(&mut self) {
        self.fireworks.borrow_mut().update();
    }
    pub fn dispatch(&mut self, kind: EventKind, x: usize, y: usize) {
        self.controller.dispatch(&self.screen, &kind, x, y);
        self.drawing_editor_controller.dispatch(&self.screen, &kind, x, y);
    }
}
