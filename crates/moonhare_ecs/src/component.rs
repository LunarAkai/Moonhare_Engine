use std::{collections::HashMap, marker::PhantomData};

use crate::generational_index::{self, GenerationalIndex};

#[derive(Debug)]
pub struct Component<T: ComponentType> {
    id: usize,
    component_type: T,
}

pub trait ComponentType {}

impl<T: ComponentType> Component<T> {
    pub fn new(id: usize, component_type: T) -> Self {
        Self { id, component_type }
    }
}
