use wasm_bindgen::prelude::*;
mod particles;

type EventKind = u8;

#[wasm_bindgen]
pub struct FireworksController {}

#[wasm_bindgen]
impl FireworksController {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FireworksController {
        FireworksController {}
    }
    pub fn dispatch(&mut self, system: &mut particles::ParticleSystem, kind: u8, x: f32, y: f32) {
        assert!(kind == 0);
        assert!(x > -1.0 && x < 1.0);
        assert!(y > -1.0 && y < 1.0);
        system.create_explosion_at(x, y);
    }
}


