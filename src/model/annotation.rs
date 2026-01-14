use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Type of text annotation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnnotationType {
    /// Custom user-defined text
    Custom,
    /// Distance measurement between two points
    Distance,
    /// Area measurement (polygon)
    Area,
    /// Radius measurement (circle/arc)
    Radius,
}

/// Text alignment for annotations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TextAlignment {
    Left,
    #[default]
    Center,
    Right,
}

/// Style configuration for text annotations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: [u8; 3], // RGB
    pub alignment: TextAlignment,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: [255, 255, 255], // White
            alignment: TextAlignment::Center,
        }
    }
}

/// Text annotation entity for labels and measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAnnotation {
    /// Position of the text
    pub position: Vector2,
    /// The displayed text
    pub text: String,
    /// Type of annotation
    pub annotation_type: AnnotationType,
    /// Style settings
    pub style: TextStyle,
    /// Anchor points for measurements (start/end for distance, polygon for area, etc.)
    pub anchor_points: Vec<Vector2>,
}

impl TextAnnotation {
    /// Create a custom text annotation
    pub fn new_custom(position: Vector2, text: String) -> Self {
        Self {
            position,
            text,
            annotation_type: AnnotationType::Custom,
            style: TextStyle::default(),
            anchor_points: Vec::new(),
        }
    }

    /// Create a distance measurement annotation
    pub fn new_distance(start: Vector2, end: Vector2) -> Self {
        let distance = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt();
        let mid = Vector2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);

        // Offset text slightly above the midpoint
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let len = (dx * dx + dy * dy).sqrt();
        let offset = if len > 0.0 {
            Vector2::new(-dy / len * 15.0, dx / len * 15.0)
        } else {
            Vector2::new(0.0, 15.0)
        };

        Self {
            position: mid + offset,
            text: format!("{:.2}", distance),
            annotation_type: AnnotationType::Distance,
            style: TextStyle {
                font_size: 12.0,
                color: [255, 200, 100], // Orange-ish for measurements
                alignment: TextAlignment::Center,
            },
            anchor_points: vec![start, end],
        }
    }

    /// Create a radius measurement annotation
    pub fn new_radius(center: Vector2, radius: f32) -> Self {
        let position = Vector2::new(center.x + radius / 2.0, center.y);

        Self {
            position,
            text: format!("R{:.2}", radius),
            annotation_type: AnnotationType::Radius,
            style: TextStyle {
                font_size: 12.0,
                color: [255, 200, 100],
                alignment: TextAlignment::Center,
            },
            anchor_points: vec![center],
        }
    }

    /// Hit test for text annotation (simplified bounding box)
    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        // Approximate text bounds based on character count and font size
        let width = self.text.len() as f32 * self.style.font_size * 0.5;
        let height = self.style.font_size;

        let half_width = width / 2.0 + tolerance;
        let half_height = height / 2.0 + tolerance;

        pos.x >= self.position.x - half_width
            && pos.x <= self.position.x + half_width
            && pos.y >= self.position.y - half_height
            && pos.y <= self.position.y + half_height
    }

    /// Recalculate measurement text if anchor points change
    pub fn recalculate(&mut self) {
        match self.annotation_type {
            AnnotationType::Distance => {
                if self.anchor_points.len() >= 2 {
                    let start = self.anchor_points[0];
                    let end = self.anchor_points[1];
                    let distance = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt();
                    self.text = format!("{:.2}", distance);

                    // Update position to midpoint
                    self.position =
                        Vector2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0 + 15.0);
                }
            }
            AnnotationType::Radius => {
                // Radius would need external radius value, skip auto-recalc
            }
            _ => {}
        }
    }
}
