use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Window instance - rectangle symbol on a host line/beam
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub host_entity_idx: usize, // Index of host Line or Beam
    pub position_t: f32,        // Position along host (0.0 - 1.0)
    pub width: f32,             // Window width
    pub height: f32,            // Window height (visual only)
}

impl Window {
    pub fn new(host_entity_idx: usize, position_t: f32, width: f32, height: f32) -> Self {
        Self {
            host_entity_idx,
            position_t: position_t.clamp(0.0, 1.0),
            width,
            height,
        }
    }

    /// Get position on host given start and end points
    pub fn get_position(&self, host_start: Vector2, host_end: Vector2) -> Vector2 {
        Vector2::new(
            host_start.x + self.position_t * (host_end.x - host_start.x),
            host_start.y + self.position_t * (host_end.y - host_start.y),
        )
    }

    /// Get the 4 corners of the window rectangle
    pub fn get_corners(&self, host_start: Vector2, host_end: Vector2) -> [Vector2; 4] {
        let pos = self.get_position(host_start, host_end);
        let host_angle = (host_end.y - host_start.y).atan2(host_end.x - host_start.x);

        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        let cos_a = host_angle.cos();
        let sin_a = host_angle.sin();

        // Local corners (along and perpendicular to host)
        let corners_local = [
            (-half_w, -half_h),
            (half_w, -half_h),
            (half_w, half_h),
            (-half_w, half_h),
        ];

        let mut world = [Vector2::new(0.0, 0.0); 4];
        for (i, (lx, ly)) in corners_local.iter().enumerate() {
            // lx is along host direction, ly is perpendicular
            let wx = pos.x + lx * cos_a - ly * sin_a;
            let wy = pos.y + lx * sin_a + ly * cos_a;
            world[i] = Vector2::new(wx, wy);
        }

        world
    }
}
