use crate::graph::edges::edge_trait::EdgeTrait;

#[derive(Copy, Clone, Default)]
pub struct SimpleEdge {
    to: u32,
}

impl SimpleEdge {
    pub fn new(to: usize) -> Self {
        Self { to: to as u32 }
    }
}

impl EdgeTrait for SimpleEdge {
    #[inline(always)]
    fn to(&self) -> usize {
        self.to as usize
    }

    fn rev(&self, from: usize) -> Self {
        Self { to: from as u32 }
    }
}
