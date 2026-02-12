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
    /// Perimeter measurement
    Perimeter,
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
    /// Rotation angle in radians
    #[serde(default)]
    pub rotation: f32,
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
            rotation: 0.0,
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
            rotation: dy.atan2(dx),
        }
    }

    /// Create an area measurement annotation
    pub fn new_area(centroid: Vector2, area: f32, polygon_points: Vec<Vector2>) -> Self {
        Self {
            position: centroid,
            text: format!("Area: {:.2}", area),
            annotation_type: AnnotationType::Area,
            style: TextStyle {
                font_size: 14.0,
                color: [100, 255, 100], // Light Green
                alignment: TextAlignment::Center,
            },
            anchor_points: polygon_points,
            rotation: 0.0,
        }
    }

    /// Create a perimeter measurement annotation
    pub fn new_perimeter(centroid: Vector2, perimeter: f32, path_points: Vec<Vector2>) -> Self {
        Self {
            position: Vector2::new(centroid.x, centroid.y - 18.0), // Offset slightly below area
            text: format!("Perim: {:.2}", perimeter),
            annotation_type: AnnotationType::Perimeter,
            style: TextStyle {
                font_size: 14.0,
                color: [100, 200, 255], // Light Blue
                alignment: TextAlignment::Center,
            },
            anchor_points: path_points,
            rotation: 0.0,
        }
    }

    /// Hit test for text annotation - simple distance-based check
    pub fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        // Calculate approximate text size
        let width = self.text.len() as f32 * self.style.font_size * 0.6;
        let height = self.style.font_size * 1.5;

        // Use distance from center for reliable detection
        let dx = (pos.x - self.position.x).abs();
        let dy = (pos.y - self.position.y).abs();

        // Generous hit area
        dx <= (width / 2.0 + tolerance + 10.0) && dy <= (height / 2.0 + tolerance + 10.0)
    }
}
