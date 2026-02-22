use super::Geometry;
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
}

impl Geometry for Circle {
    fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        let d = pos.dist(self.center);
        if self.filled {
            d <= self.radius + tolerance
        } else {
            (d - self.radius).abs() < tolerance
        }
    }

    fn bounding_box(&self) -> (Vector2, Vector2) {
        (
            Vector2::new(self.center.x - self.radius, self.center.y - self.radius),
            Vector2::new(self.center.x + self.radius, self.center.y + self.radius),
        )
    }

    fn as_polyline(&self) -> Vec<Vector2> {
        let segments = 32;
        (0..=segments)
            .map(|i| {
                let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
                Vector2::new(
                    self.center.x + self.radius * angle.cos(),
                    self.center.y + self.radius * angle.sin(),
                )
            })
            .collect()
    }

    fn is_closed(&self) -> bool {
        true
    }

    fn is_filled(&self) -> bool {
        self.filled
    }
}
