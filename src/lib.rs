use wasm_bindgen::prelude::*;

mod model;
use model::Model;

pub fn create() -> Model {
    let model = Model::new();
    model
}

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let m = create();
    }
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
