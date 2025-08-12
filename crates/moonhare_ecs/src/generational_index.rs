#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct GenerationalIndex {
    index: usize,
    generation: u64,
}

impl GenerationalIndex {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

#[derive(Debug, Clone)]
struct AllocatorEntry {
    is_live: bool,
    generation: u64,
}

#[derive(Debug, Clone, Default)]
pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl GenerationalIndexAllocator {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn allocate(&mut self) -> GenerationalIndex {
        match self.free.pop() {
            Some(index) => {
                self.entries[index].generation += 1;
                self.entries[index].is_live = true;

                GenerationalIndex {
                    index,
                    generation: self.entries[index].generation,
                }
            }
            None => {
                self.entries.push(AllocatorEntry {
                    is_live: true,
                    generation: 0,
                });

                GenerationalIndex {
                    index: self.entries.len() - 1,
                    generation: 0,
                }
            }
        }
    }

    // Returns true if the index was allocated before and is now deallocated
    pub fn deallocate(&mut self, index: GenerationalIndex) -> bool {
        if self.is_live(index) {
            self.entries[index.index()].is_live = false;
            self.free.push(index.index());
            true
        } else {
            false
        }
    }

    pub fn is_live(&self, index: GenerationalIndex) -> bool {
        index.index() < self.entries.len()
            && self.entries[index.index()].generation == index.generation
            && self.entries[index.index()].is_live
    }

    pub fn max_allocated_index(&self) -> usize {
        self.entries.len()
    }
}

struct ArrayEntry<T> {
    value: T,
    generation: u64,
}

pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn new() -> GenerationalIndexArray<T> {
        GenerationalIndexArray(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn insert(&mut self, index: GenerationalIndex, value: T) {
        while self.0.len() <= index.index() {
            self.0.push(None);
        }

        let previous_generation = match &self.0[index.index()] {
            Some(entry) => entry.generation,
            None => 0,
        };

        if previous_generation > index.generation() {
            panic!("write an index from previous gen");
        }

        self.0[index.index()] = Some(ArrayEntry {
            value,
            generation: index.generation(),
        });
    }

    pub fn remove(&mut self, index: GenerationalIndex) {
        if index.index() < self.0.len() {
            self.0[index.index()] = None;
        }
    }

    pub fn get(&self, index: GenerationalIndex) -> Option<&T> {
        if index.index() >= self.0.len() {
            return None;
        }

        match &self.0[index.index()] {
            Some(entry) => {
                if entry.generation == index.generation() {
                    Some(&entry.value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
    pub fn get_mut(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        if index.index() >= self.0.len() {
            return None;
        }

        match &mut self.0[index.index()] {
            Some(entry) => {
                if entry.generation == index.generation() {
                    Some(&mut entry.value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
