use crate::nodes::node::Node;

#[derive(Debug, Clone)]
pub struct World {
    nodes: Vec<Box<dyn Node>>
}

impl World {
    pub fn new() -> Self {
        Self { 
            nodes: vec![]
        }
    }

    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node)
    }

    pub fn init(self) {
        for mut node in self.nodes {
            node.init();
        }
    }

    pub fn update(self) {
        for mut node in self.nodes {
            node.update();
        }
    }
} 