use std::fmt::Error;

use anymap::AnyMap;

use crate::{Entity, ECS};

/// stores Entitys, Components and resources
/// provides methods to search for specific Entitys 
#[derive(Debug)]
pub struct World {
    ecs: ECS,
    resources: AnyMap
}

impl World {
    pub fn new() -> Self {
        Self { 
            ecs: ECS::new(),
            resources: AnyMap::new(), 
        }
    }
}