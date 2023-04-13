use crate::events::*;

pub type Vector2 = [f32; 2];
pub type Vector4 = [f32; 4];
pub type Color = Vector4;


pub struct Screen {
    pub width: usize,
    pub height: usize,
}

pub trait Controller {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize);
}

pub struct Action {
    pub kind: EventKind,
    pub x: f32,
    pub y: f32,
}

pub trait Model {
    fn buffers(&self) -> Vec<(*const f32)>;
    fn update(&self);
    fn act(&self, action: Action);
}