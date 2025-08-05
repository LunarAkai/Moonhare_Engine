use moonhare_derives::Node;

pub trait Node {
    fn init(&mut self);
    fn update(&mut self);
}

impl<T> Node for Box<T> {
    fn init(&mut self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}
