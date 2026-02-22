use crate::model::Vector2;
use crate::model::shapes::Geometry;
use serde::{Deserialize, Serialize};

/// Anchor alignment for the beam body relative to its axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeamAnchor {
    Center,
    Top,
    Bottom,
}

/// Data defining an individual structural beam instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeamData {
    /// Start point
    pub start: Vector2,
    /// End point
    pub end: Vector2,
    /// ID of the BeamType definition.
    pub beam_type_id: u64,
    /// Text label (e.g., "K101").
    pub label: String,
    /// Anchor alignment
    #[serde(default = "default_anchor")]
    pub anchor: BeamAnchor,
}

fn default_anchor() -> BeamAnchor {
    BeamAnchor::Center
}

impl BeamData {
    pub fn new(
        start: Vector2,
        end: Vector2,
        beam_type_id: u64,
        label: String,
        anchor: BeamAnchor,
    ) -> Self {
        Self {
            start,
            end,
            beam_type_id,
            label,
            anchor,
        }
    }

    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }
}

impl Geometry for BeamData {
    fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        // We don't have width here easily without BeamType,
        // but for hit test we can use a default or handle it in CadModel.
        // Actually, Geometry methods should be self-contained if possible.
        // Let's assume a reasonable hit width for now or pass it if we refactor.
        // For Beams, we hit test against the line segment with tolerance.
        let l2 = (self.start - self.end).length_squared();
        if l2 == 0.0 {
            return (pos - self.start).length() < tolerance;
        }
        let t = ((pos - self.start).dot(self.end - self.start) / l2).clamp(0.0, 1.0);
        let projection = self.start + (self.end - self.start) * t;
        (pos - projection).length() < tolerance + 15.0 // Buffer for beam thickness
    }

    fn bounding_box(&self) -> (Vector2, Vector2) {
        let min_x = self.start.x.min(self.end.x) - 20.0;
        let min_y = self.start.y.min(self.end.y) - 20.0;
        let max_x = self.start.x.max(self.end.x) + 20.0;
        let max_y = self.start.y.max(self.end.y) + 20.0;
        (Vector2::new(min_x, min_y), Vector2::new(max_x, max_y))
    }

    fn as_polyline(&self) -> Vec<Vector2> {
        vec![self.start, self.end]
    }

    fn is_closed(&self) -> bool {
        false
    }

    fn is_filled(&self) -> bool {
        false
    }
}
