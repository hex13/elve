use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum EventKind {
    PointerDown = 1,
    PointerMove = 2,
    PointerUp = 3,
    TogglePlay = 100,
    ChangeColor = 200,
    Interact = 300,
    // actions
    Draw = 1000,
    DrawLine = 1001,
    DrawRectangle = 1002,
}

