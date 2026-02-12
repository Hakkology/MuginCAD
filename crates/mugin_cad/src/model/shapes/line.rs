use super::Geometry;
use crate::model::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub start: Vector2,
    pub end: Vector2,
    /// Whether to show the length label
    #[serde(default)]
    pub show_length: bool,
    /// Offset of the label from the midpoint (for dragging/repositioning)
    #[serde(default)]
    pub label_offset: Vector2,
}

impl Line {
    pub fn new(start: Vector2, end: Vector2) -> Self {
        Self {
            start,
            end,
            show_length: false,
            label_offset: Vector2::new(0.0, 0.0), // Default 0, relative to smart offset
        }
    }

    /// Calculate the smart offset vector (visually "up") based on line orientation
    pub fn calculate_smart_offset(&self, tolerance: f32) -> Vector2 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let len = (dx * dx + dy * dy).sqrt();

        if len < 0.001 {
            return Vector2::new(0.0, 15.0); // Fallback
        }

        // 15.0 pixels screen distance ~ 3.0 * tolerance world distance
        let offset_dist = 3.0 * tolerance;

        // Normal check
        let mut nx = -dy / len;
        let mut ny = dx / len;

        // Force "Up" (World Y positive)
        if ny < 0.0 {
            nx = -nx;
            ny = -ny;
        }

        Vector2::new(nx * offset_dist, ny * offset_dist)
    }

    /// Check if a point hits the label specifically
    pub fn hit_test_label(&self, pos: Vector2, tolerance: f32) -> bool {
        if !self.show_length {
            return false;
        }

        let smart_offset = self.calculate_smart_offset(tolerance);
        let mid = self.midpoint();
        // Position = Mid + Smart + UserOffset
        let label_pos = mid + smart_offset + self.label_offset;

        let len = self.length();
        let text_len = format!("{:.2}", len).len();
        let box_w = text_len as f32 * 1.4 * tolerance;
        let box_h = 2.8 * tolerance;

        let dx = (pos.x - label_pos.x).abs();
        let dy = (pos.y - label_pos.y).abs();

        dx < box_w / 2.0 + tolerance && dy < box_h / 2.0 + tolerance
    }

    /// Get the length of the line
    pub fn length(&self) -> f32 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Get the midpoint of the line
    pub fn midpoint(&self) -> Vector2 {
        Vector2::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }
}

impl Geometry for Line {
    fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        // Basic line hit test
        if pos.dist_to_line(self.start, self.end) < tolerance {
            return true;
        }

        // label hit test
        self.hit_test_label(pos, tolerance)
    }

    fn center(&self) -> Vector2 {
        self.midpoint()
    }

    fn bounding_box(&self) -> (Vector2, Vector2) {
        (
            Vector2::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y)),
            Vector2::new(self.start.x.max(self.end.x), self.start.y.max(self.end.y)),
        )
    }

    fn as_polyline(&self) -> Vec<Vector2> {
        vec![self.start, self.end]
    }

    fn translate(&mut self, delta: Vector2) {
        self.start = self.start + delta;
        self.end = self.end + delta;
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
        self.start = rotate_point(self.start);
        self.end = rotate_point(self.end);
    }

    fn scale(&mut self, base: Vector2, factor: f32) {
        let scale_point = |p: Vector2| -> Vector2 {
            Vector2::new(
                base.x + (p.x - base.x) * factor,
                base.y + (p.y - base.y) * factor,
            )
        };
        self.start = scale_point(self.start);
        self.end = scale_point(self.end);
    }

    fn is_closed(&self) -> bool {
        false
    }

    fn is_filled(&self) -> bool {
        false
    }
}
