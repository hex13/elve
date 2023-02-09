use wasm_bindgen::prelude::*;
mod particles;

type EventKind = u8;

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
    pub fn dispatch(&mut self, system: &mut particles::ParticleSystem, kind: EventKind, x: f32, y: f32) {
        match kind {
            PointerDown => {
                system.create_explosion_at(x, y);
                self.pointer_down = true;
            }
            PointerMove => {
                if self.pointer_down {
                    system.create_explosion_at(x, y);
                }
            }
            PointerUp => {
                self.pointer_down = false;
            }
            _ => ()
        }
    }
}


