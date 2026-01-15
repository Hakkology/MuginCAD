use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Beam instance - connects two points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beam {
    pub type_id: String, // Reference to BeamType
    pub start: Vector2,  // Start position
    pub end: Vector2,    // End position
    pub index: u32,      // Instance number
}

impl Beam {
    pub fn new(type_id: String, start: Vector2, end: Vector2, index: u32) -> Self {
        Self {
            type_id,
            start,
            end,
            index,
        }
    }

    /// Get display name like "25x40-B1"
    pub fn display_name(&self) -> String {
        format!("{}-B{}", self.type_id, self.index)
    }

    /// Get center point
    pub fn center(&self) -> Vector2 {
        Vector2::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }

    /// Get beam length (span)
    pub fn length(&self) -> f32 {
        self.start.dist(self.end)
    }

    /// Get beam angle
    pub fn angle(&self) -> f32 {
        (self.end.y - self.start.y).atan2(self.end.x - self.start.x)
    }

    /// Hit test for beam
    pub fn hit_test(&self, pos: Vector2, tolerance: f32, width: f32) -> bool {
        // Distance from point to line segment
        let line_vec = self.end - self.start;
        let line_len_sq = line_vec.x * line_vec.x + line_vec.y * line_vec.y;

        if line_len_sq < 0.0001 {
            return self.start.dist(pos) <= tolerance + width / 2.0;
        }

        let t = ((pos.x - self.start.x) * line_vec.x + (pos.y - self.start.y) * line_vec.y)
            / line_len_sq;
        let t = t.clamp(0.0, 1.0);

        let closest = Vector2::new(self.start.x + t * line_vec.x, self.start.y + t * line_vec.y);

        closest.dist(pos) <= tolerance + width / 2.0
    }

    /// Get the 4 corners of the beam rectangle in world coordinates
    pub fn get_corners(&self, width: f32) -> [Vector2; 4] {
        let angle = self.angle();
        let half_w = width / 2.0;

        // Perpendicular direction
        let perp = Vector2::new(-angle.sin(), angle.cos());

        [
            self.start - perp * half_w,
            self.start + perp * half_w,
            self.end + perp * half_w,
            self.end - perp * half_w,
        ]
    }
}
