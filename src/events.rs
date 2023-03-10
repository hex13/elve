use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum EventKind {
    PointerDown = 1,
    PointerMove = 2,
    PointerUp = 3,
    TogglePlay = 100,
    ChangeColor = 200,
}

