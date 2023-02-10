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
    particle_system: particles::ParticleSystem,
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
    
    pub fn particle_system(&mut self ) -> *const particles::ParticleSystem {
        &self.particle_system
    }
    pub fn positions(&self) -> *const f32 {
        self.particle_system.positions()
    }
    pub fn colors(&self) -> *const f32 {
        self.particle_system.colors()
    }
    pub fn create_explosion(&mut self) {
        self.particle_system.create_explosion()
    }
    pub fn update(&mut self) {
        self.particle_system.update()
    }
    pub fn set_autoexplosions(&mut self, value: bool) {
        self.particle_system.set_autoexplosions(value)
    }
    pub fn create_explosion_at(&mut self, x: f32, y: f32) {
        self.particle_system.create_explosion_at(x, y)
    }
}


#[wasm_bindgen]
pub fn create_fireworks_app(count: usize) -> App {
    let particle_system = particles::ParticleSystem::new(count);
    let resources: Vec<Resource> = vec![
        // Resource::ParticleSystem(particle_system)
    ];
    App {
        resources,
        particle_system,
    }
}
