use crate::model::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rectangle {
    pub min: Vector2,
    pub max: Vector2,
    pub filled: bool,
}

impl Rectangle {
    pub fn new(min: Vector2, max: Vector2, filled: bool) -> Self {
        Self { min, max, filled }
    }

    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        let inside = pos.x >= self.min.x - tolerance
            && pos.x <= self.max.x + tolerance
            && pos.y >= self.min.y - tolerance
            && pos.y <= self.max.y + tolerance;

        if self.filled {
            inside
        } else {
            // Check if near edges
            let near_x =
                (pos.x - self.min.x).abs() < tolerance || (pos.x - self.max.x).abs() < tolerance;
            let near_y =
                (pos.y - self.min.y).abs() < tolerance || (pos.y - self.max.y).abs() < tolerance;
            inside && (near_x || near_y)
        }
    }
}
