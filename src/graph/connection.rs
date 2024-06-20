use bevy::prelude::*;
use crate::graph::grid_transform::GridTransform;

#[derive(Component)]
pub struct Connection {
    pub a: GridTransform,
    pub b: GridTransform,
}

impl Connection {
    pub fn new(a: GridTransform, b: GridTransform) -> Self {
        Self {
            a,
            b,
        }
    }
}
