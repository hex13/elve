pub struct Model {
   pub counter: i32,
}

impl Model {
    pub fn new() -> Model {
        let m = Model {counter: 123};
        assert_eq!(m.counter, 123);
        m
    }
}