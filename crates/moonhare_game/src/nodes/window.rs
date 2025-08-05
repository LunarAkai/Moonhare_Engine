use moonhare_derives::Node;
use crate::nodes::node::Node;

#[derive(Node)]
struct Window {
    a: Box<String>
}

impl Window {
    fn init(&mut self) {

    }

    fn update(&mut self) {

    }
}