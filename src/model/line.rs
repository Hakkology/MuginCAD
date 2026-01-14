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
            label_offset: Vector2::new(0.0, -15.0), // Default offset above line
        }
    }

    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        pos.dist_to_line(self.start, self.end) < tolerance
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

    /// Get the label position (midpoint + offset)
    pub fn label_position(&self) -> Vector2 {
        self.midpoint() + self.label_offset
    }
}
