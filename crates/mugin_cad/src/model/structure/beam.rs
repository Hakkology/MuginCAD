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

    pub fn angle(&self) -> f32 {
        let diff = self.end - self.start;
        diff.y.atan2(diff.x)
    }

    pub fn get_corners(&self, width: f32) -> [Vector2; 4] {
        let dir = (self.end - self.start).normalized();
        let perp = Vector2::new(-dir.y, dir.x);

        let offset = match self.anchor {
            BeamAnchor::Center => 0.0,
            BeamAnchor::Top => width / 2.0,
            BeamAnchor::Bottom => -width / 2.0,
        };

        let half_w = width / 2.0;
        let p1 = self.start + perp * (offset + half_w);
        let p2 = self.end + perp * (offset + half_w);
        let p3 = self.end + perp * (offset - half_w);
        let p4 = self.start + perp * (offset - half_w);

        [p1, p2, p3, p4]
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

    fn center(&self) -> Vector2 {
        (self.start + self.end) * 0.5
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

    fn translate(&mut self, delta: Vector2) {
        self.start = self.start + delta;
        self.end = self.end + delta;
    }

    fn rotate(&mut self, pivot: Vector2, angle: f32) {
        let rot = |p: Vector2| {
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            let dx = p.x - pivot.x;
            let dy = p.y - pivot.y;
            Vector2::new(
                pivot.x + dx * cos_a - dy * sin_a,
                pivot.y + dx * sin_a + dy * cos_a,
            )
        };
        self.start = rot(self.start);
        self.end = rot(self.end);
    }

    fn scale(&mut self, base: Vector2, factor: f32) {
        self.start = base + (self.start - base) * factor;
        self.end = base + (self.end - base) * factor;
    }

    fn is_closed(&self) -> bool {
        false
    }

    fn is_filled(&self) -> bool {
        false
    }
}
