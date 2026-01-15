use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Flooring instance - filled area bounded by beams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flooring {
    pub type_id: String,               // Reference to FloorType
    pub boundary_points: Vec<Vector2>, // Polygon vertices
    pub index: u32,
}

impl Flooring {
    pub fn new(type_id: String, boundary_points: Vec<Vector2>, index: u32) -> Self {
        Self {
            type_id,
            boundary_points,
            index,
        }
    }

    /// Get display name like "150mm-F1"
    pub fn display_name(&self) -> String {
        format!("{}-F{}", self.type_id, self.index)
    }

    /// Calculate centroid
    pub fn center(&self) -> Vector2 {
        if self.boundary_points.is_empty() {
            return Vector2::new(0.0, 0.0);
        }
        let sum: Vector2 = self
            .boundary_points
            .iter()
            .fold(Vector2::new(0.0, 0.0), |acc, p| acc + *p);
        sum / self.boundary_points.len() as f32
    }

    /// Calculate area using shoelace formula
    pub fn area(&self) -> f32 {
        let n = self.boundary_points.len();
        if n < 3 {
            return 0.0;
        }
        let mut area = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            area += self.boundary_points[i].x * self.boundary_points[j].y;
            area -= self.boundary_points[j].x * self.boundary_points[i].y;
        }
        (area / 2.0).abs()
    }

    /// Calculate perimeter
    pub fn perimeter(&self) -> f32 {
        let n = self.boundary_points.len();
        if n < 2 {
            return 0.0;
        }
        let mut perim = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            perim += self.boundary_points[i].dist(self.boundary_points[j]);
        }
        perim
    }

    /// Hit test - point in polygon
    pub fn hit_test(&self, pos: Vector2, _tolerance: f32) -> bool {
        let n = self.boundary_points.len();
        if n < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = n - 1;
        for i in 0..n {
            let pi = self.boundary_points[i];
            let pj = self.boundary_points[j];

            if ((pi.y > pos.y) != (pj.y > pos.y))
                && (pos.x < (pj.x - pi.x) * (pos.y - pi.y) / (pj.y - pi.y) + pi.x)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}
