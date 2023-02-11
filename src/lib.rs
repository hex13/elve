use wasm_bindgen::prelude::*;
mod particles;
mod drawing_editor;
use drawing_editor::*;

#[wasm_bindgen]
pub enum EventKind {
    PointerDown = 1,
    PointerMove = 2,
    PointerUp = 3,
    TogglePlay = 100,
}


#[wasm_bindgen]
pub struct FireworksController {
    pointer_down: bool,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=window)]
    pub fn pass_firework_buffers(positions: *const f32, colors: *const f32 );
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: String);
}


#[wasm_bindgen]
impl FireworksController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FireworksController {
        FireworksController {pointer_down: false}
    }
    pub fn dispatch(&mut self, model: &mut ParticleSystemModel, kind: EventKind, x: f32, y: f32) {
        match kind {
            EventKind::PointerDown => {
                model.create_explosion_at(x, y);
                self.pointer_down = true;
            }
            EventKind::PointerMove => {
                if self.pointer_down {
                    model.create_explosion_at(x, y);
                }
            }
            EventKind::PointerUp => {
                self.pointer_down = false;
            }
            EventKind::TogglePlay => {
                model.togglePlay();
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

#[wasm_bindgen]
struct App {
    fireworks: ParticleSystemModel,
    drawing_editor: drawing_editor::DrawingEditor,
    controller: FireworksController,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> App {
        App {
            fireworks: create_fireworks_model(3000),
            controller: FireworksController::new(),
            drawing_editor: drawing_editor::DrawingEditor::new(width, height),
        }
    }
    pub fn drawing_editor_pixels(&self) -> *const u8 {
        &self.drawing_editor.pixels[0]
    }
    pub fn update(&mut self) {
        self.fireworks.update();
    }
    // TODO make coordinates uniform between fireworks and drawing editor
    // to eliminate need of having to separate pairs of coords
    pub fn dispatch(&mut self, kind: EventKind, x: f32, y: f32, x2: usize, y2: usize) {
        self.controller.dispatch(&mut self.fireworks, kind, x, y);
        self.drawing_editor.draw(x2, y2);
    }
}
