use super::Geometry;
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
}

impl Geometry for Rectangle {
    fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
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

    fn center(&self) -> Vector2 {
        Vector2::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
        )
    }

    fn bounding_box(&self) -> (Vector2, Vector2) {
        (
            Vector2::new(self.min.x.min(self.max.x), self.min.y.min(self.max.y)),
            Vector2::new(self.min.x.max(self.max.x), self.min.y.max(self.max.y)),
        )
    }

    fn as_polyline(&self) -> Vec<Vector2> {
        vec![
            self.min,
            Vector2::new(self.max.x, self.min.y),
            self.max,
            Vector2::new(self.min.x, self.max.y),
            self.min,
        ]
    }

    fn translate(&mut self, delta: Vector2) {
        self.min = self.min + delta;
        self.max = self.max + delta;
    }

    fn rotate(&mut self, pivot: Vector2, angle: f32) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let rotate_point = |p: Vector2| -> Vector2 {
            let dx = p.x - pivot.x;
            let dy = p.y - pivot.y;
            Vector2::new(
                pivot.x + dx * cos_a - dy * sin_a,
                pivot.y + dx * sin_a + dy * cos_a,
            )
        };

        // Rotating an AABB results in a larger AABB (axis-aligned bounding box of the rotated shape)
        let p1 = rotate_point(self.min);
        let p2 = rotate_point(Vector2::new(self.max.x, self.min.y));
        let p3 = rotate_point(self.max);
        let p4 = rotate_point(Vector2::new(self.min.x, self.max.y));

        self.min = Vector2::new(
            p1.x.min(p2.x).min(p3.x).min(p4.x),
            p1.y.min(p2.y).min(p3.y).min(p4.y),
        );
        self.max = Vector2::new(
            p1.x.max(p2.x).max(p3.x).max(p4.x),
            p1.y.max(p2.y).max(p3.y).max(p4.y),
        );
    }

    fn scale(&mut self, base: Vector2, factor: f32) {
        let scale_point = |p: Vector2| -> Vector2 {
            Vector2::new(
                base.x + (p.x - base.x) * factor,
                base.y + (p.y - base.y) * factor,
            )
        };
        self.min = scale_point(self.min);
        self.max = scale_point(self.max);
    }

    fn is_closed(&self) -> bool {
        true
    }

    fn is_filled(&self) -> bool {
        self.filled
    }
}
