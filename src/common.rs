use crate::events::*;

pub struct Screen {
    pub width: usize,
    pub height: usize,
}

pub trait Controller {
    fn dispatch(&mut self, screen: &Screen, kind: &EventKind, x: usize, y: usize);
}