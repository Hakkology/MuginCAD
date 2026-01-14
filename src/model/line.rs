use crate::model::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub start: Vector2,
    pub end: Vector2,
}

impl Line {
    pub fn new(start: Vector2, end: Vector2) -> Self {
        Self { start, end }
    }

    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        pos.dist_to_line(self.start, self.end) < tolerance
    }
}
