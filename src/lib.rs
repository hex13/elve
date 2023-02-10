use wasm_bindgen::prelude::*;
mod particles;
mod drawing_editor;

type EventKind = u8;

// TODO convert to enums because C style enums are supported in wasm_bindgen
const PointerDown: EventKind = 1;
const PointerMove: EventKind = 2;
const PointerUp: EventKind = 3;

#[wasm_bindgen]
pub struct FireworksController {
    pointer_down: bool,
}

#[wasm_bindgen]
impl FireworksController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FireworksController {
        FireworksController {pointer_down: false}
    }
    pub fn dispatch(&mut self, app: &mut App, kind: EventKind, x: f32, y: f32) {
        match kind {
            PointerDown => {
                app.create_explosion_at(x, y);
                self.pointer_down = true;
            }
            PointerMove => {
                if self.pointer_down {
                    app.create_explosion_at(x, y);
                }
            }
            PointerUp => {
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
pub struct App {
    resources: Vec<Resource>,
    particle_system_state: particles::ParticleSystemState,
}

#[wasm_bindgen]
impl App {
    // #[wasm_bindgen(constructor)]
    // pub fn new() -> App {
    //     App {
    //         resources: Vec::new(),
    //     }
    // }

    // pub fn add_resource(&mut self, kind: ResourceKind) -> Handle {
    //     match kind {
    //         ResourceKind::ParticleSystem => {
    //             self.resources.push(Resource::ParticleSystem(particles::ParticleSystem::new()));
    //             (self.resources.len() - 1) as u32
    //         }
    //         _ => 0
    //     }
    // }
    
    pub fn positions(&self) -> *const f32 {
        &self.particle_system_state.positions[0]
    }
    pub fn colors(&self) -> *const f32 {
        &self.particle_system_state.colors[0]
    }
    pub fn update(&mut self) {
        particles::ParticleSystem::update(&mut self.particle_system_state)
    }
    pub fn set_autoexplosions(&mut self, value: bool) {
        self.particle_system_state.autoexplosions = value;
    }
    pub fn create_explosion_at(&mut self, x: f32, y: f32) {
        particles::ParticleSystem::create_explosion_at(&mut self.particle_system_state, x, y)
    }
}


#[wasm_bindgen]
pub fn create_fireworks_app(count: usize) -> App {

    let unit = 0.2;
    let positions = vec![0.0; count * 2];
    let velocities = vec![0.0; count * 2];
    let colors = vec![0.0; count * 4];
    let explosions = Vec::new();
    let particle_system_state = particles::ParticleSystemState { count, positions, velocities, colors, explosions, autoexplosions: false };

    let resources: Vec<Resource> = vec![
        // Resource::ParticleSystem(particle_system)
    ];
    App {
        resources,
        particle_system_state,
    }
}
