use crate::model::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub center: Vector2,
    pub radius: f32,
    pub filled: bool,
}

impl Circle {
    pub fn new(center: Vector2, radius: f32, filled: bool) -> Self {
        Self {
            center,
            radius,
            filled,
        }
    }

    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        let d = pos.dist(self.center);
        if self.filled {
            d <= self.radius + tolerance
        } else {
            (d - self.radius).abs() < tolerance
        }
    }
}
