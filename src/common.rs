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
    fn transform_input(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize) -> Option<Action> {
        None
    }
}

pub enum ActionPayload {
    DrawData(usize, usize, usize, usize, usize, Color),
    Empty,
}
pub struct Action {
    pub kind: EventKind,
    pub x: f32,
    pub y: f32,
    pub payload: ActionPayload,
}

pub trait Model {
    fn buffers(&self) -> Vec<(*const f32, usize)>;
    fn update(&self);
    fn act(&mut self, action: &Action) {}
}