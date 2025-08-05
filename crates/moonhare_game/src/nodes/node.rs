use dyn_clone::DynClone;
use moonhare_derives::Node;
use std::fmt::{Debug, Formatter, Result};

pub trait Node: DynClone {
    fn init(&mut self);
    fn update(&mut self);
}
dyn_clone::clone_trait_object!(Node);




impl Debug for dyn Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "")
    } 
}

impl<T: Clone> Node for Box<T> {
    fn init(&mut self) {
      
    }

    fn update(&mut self) {

    }
}
