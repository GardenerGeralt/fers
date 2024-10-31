use std::ops;
use crate::element::*;

pub struct Node {
    x: f32,
    y: f32,
    pub nr: usize
}

impl<'a> ops::Sub<&'a Node> for &Node {
    type Output = Element1D<'a>;

    fn sub(self, rhs: &Node) -> Self::Output {
        Element1D::new(rhs, self, None)
    }
}

impl Node {
    pub fn new(x: f32, y: f32, nr:usize) -> Node {
        Node {x, y, nr}
    }

    pub fn xy(&self) -> [f32; 2] {
        [self.x.clone(), self.y.clone()]
    }
}