use crate::graph::edges::edge_trait::EdgeTrait;
use crate::misc::num_traits::Number;

#[derive(Copy, Clone)]
pub struct SimpleEdge {
    to: u32,
}

impl SimpleEdge {
    pub fn new(to: usize) -> Self {
        Self { to: to as u32 }
    }
}

impl EdgeTrait for SimpleEdge {
    fn to(&self) -> usize {
        self.to as usize
    }
}
