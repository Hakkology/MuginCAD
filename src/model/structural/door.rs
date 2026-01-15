use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Door swing direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum DoorSwing {
    #[default]
    LeftIn,
    LeftOut,
    RightIn,
    RightOut,
}

/// Door instance - symbol placed on a host line/beam
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Door {
    pub host_entity_idx: usize, // Index of host Line or Beam
    pub position_t: f32,        // Position along host (0.0 - 1.0)
    pub width: f32,             // Door opening width
    pub swing: DoorSwing,
}

impl Door {
    pub fn new(host_entity_idx: usize, position_t: f32, width: f32) -> Self {
        Self {
            host_entity_idx,
            position_t: position_t.clamp(0.0, 1.0),
            width,
            swing: DoorSwing::LeftIn,
        }
    }

    /// Get position on host given start and end points
    pub fn get_position(&self, host_start: Vector2, host_end: Vector2) -> Vector2 {
        Vector2::new(
            host_start.x + self.position_t * (host_end.x - host_start.x),
            host_start.y + self.position_t * (host_end.y - host_start.y),
        )
    }

    /// Calculate the door panel line and arc for rendering
    pub fn get_geometry(
        &self,
        host_start: Vector2,
        host_end: Vector2,
    ) -> (Vector2, Vector2, f32, f32) {
        let pos = self.get_position(host_start, host_end);
        let host_angle = (host_end.y - host_start.y).atan2(host_end.x - host_start.x);

        let perp_angle = match self.swing {
            DoorSwing::LeftIn | DoorSwing::LeftOut => host_angle + std::f32::consts::FRAC_PI_2,
            DoorSwing::RightIn | DoorSwing::RightOut => host_angle - std::f32::consts::FRAC_PI_2,
        };

        let door_end = Vector2::new(
            pos.x + self.width * perp_angle.cos(),
            pos.y + self.width * perp_angle.sin(),
        );

        let arc_start = match self.swing {
            DoorSwing::LeftIn | DoorSwing::RightIn => host_angle,
            DoorSwing::LeftOut | DoorSwing::RightOut => host_angle + std::f32::consts::PI,
        };

        let arc_end = arc_start + std::f32::consts::FRAC_PI_2;

        (pos, door_end, arc_start, arc_end)
    }
}
