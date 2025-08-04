use crate::{entity, generational_index::{GenerationalIndex, GenerationalIndexAllocator}};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Entity(GenerationalIndex);

impl Entity {
    pub fn index(&self) -> usize {
        self.0.index()
    }

    pub fn generation(&self) -> u64 {
        self.0.generation()
    }
}


#[derive(Debug, Clone)]
pub struct EntityAllocator(GenerationalIndexAllocator);

impl EntityAllocator {
    pub fn new() -> Self {
        EntityAllocator(GenerationalIndexAllocator::new())
    }

    pub fn allocate(&mut self) -> Entity {
        Entity(self.0.allocate())
    }

    pub fn deallocate(&mut self, entity: Entity) -> bool {
        self.0.deallocate(entity.0)
    }

    pub fn is_live(&self, entity: Entity) -> bool {
        self.0.is_live(entity.0)
    }
}