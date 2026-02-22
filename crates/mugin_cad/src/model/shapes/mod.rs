pub mod annotation;
pub mod arc;
pub mod circle;
pub mod line;
pub mod rectangle;

use crate::model::Vector2;

/// Common geometric operations for all shapes.
pub trait Geometry {
    /// Check if a point interacts with the shape within a tolerance.
    fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool;

    /// Get the axis-aligned bounding box (min, max).
    fn bounding_box(&self) -> (Vector2, Vector2);

    /// Convert to a sequence of points (for rendering or analysis).
    fn as_polyline(&self) -> Vec<Vector2>;

    /// Whether the shape is a closed loop.
    fn is_closed(&self) -> bool;

    /// Whether the shape has a fill.
    fn is_filled(&self) -> bool;
}
