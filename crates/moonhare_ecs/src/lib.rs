use anymap::AnyMap;

use crate::generational_index::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexArray};

pub mod generational_index;

pub type Entity = GenerationalIndex;
pub type EntityMap<T> = GenerationalIndexArray<T>;

// based on: https://kyren.github.io/2018/09/14/rustconf-talk.html
pub struct ECS {
    pub entity_allocator: GenerationalIndexAllocator,
    pub entity_components: AnyMap,
    pub resources: AnyMap
}