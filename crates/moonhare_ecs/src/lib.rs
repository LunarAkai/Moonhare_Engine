use std::{
    collections::HashMap,
    path::{Component, Components},
};

use anymap::AnyMap;

use crate::{
    entity::{Entity, EntityAllocator},
    generational_index::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexArray},
};

pub mod component;
pub mod entity;
pub mod generational_index;
pub mod world;

// based on: https://kyren.github.io/2018/09/14/rustconf-talk.html

/* Moonhare ECS Design
--------------------------------------
                 Game
                  🠟
                Systems
(RenderSystem, PhysicsSystem, EnemyAISystem, EnemyCollisionSystem,...)
                  🠟
                Entity
                  🠟
               Components
--------------------------------------
*/

#[derive(Debug)]
pub struct ECS {
    entities: EntityAllocator,
    components: AnyMap,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: EntityAllocator::new(),
            components: AnyMap::new(),
        }
    }

    pub fn add_entity(&mut self) -> Entity {
        self.entities.allocate()
    }

    pub fn entity_is_live(&self, entity: Entity) -> bool {
        self.entities.is_live(entity)
    }

    pub fn register_component() {}
}
