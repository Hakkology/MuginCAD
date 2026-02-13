use crate::model::Vector2;
use crate::model::shapes::Geometry;
use serde::{Deserialize, Serialize};

/// Anchor point for the column instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnAnchor {
    Center,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

/// Data defining a structural column.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnData {
    /// Center position of the column.
    pub center: Vector2,
    /// Width of the column (along X-axis before rotation).
    pub width: f32,
    /// Height/Depth of the column (along Y-axis before rotation).
    pub height: f32,
    /// Rotation in radians.
    pub rotation: f32,
    /// ID of the ColumnType definition this instance is based on.
    pub column_type_id: u64,
    /// Text label displayed on the column (e.g., "S1").
    pub label: String,
    /// The geometric anchor used for placement and resizing.
    #[serde(default = "default_anchor")]
    pub anchor: ColumnAnchor,
}

fn default_anchor() -> ColumnAnchor {
    ColumnAnchor::Center
}

impl ColumnData {
    pub fn new(
        center: Vector2,
        width: f32,
        height: f32,
        column_type_id: u64,
        label: String,
        anchor: ColumnAnchor,
    ) -> Self {
        Self {
            center,
            width,
            height,
            rotation: 0.0,
            column_type_id,
            label,
            anchor,
        }
    }

    /// Helper to get corner points (rotated).
    pub fn get_corners(&self) -> [Vector2; 4] {
        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        let cos_a = self.rotation.cos();
        let sin_a = self.rotation.sin();

        let rotate = |x: f32, y: f32| {
            Vector2::new(
                self.center.x + x * cos_a - y * sin_a,
                self.center.y + x * sin_a + y * cos_a,
            )
        };

        [
            rotate(-half_w, -half_h),
            rotate(half_w, -half_h),
            rotate(half_w, half_h),
            rotate(-half_w, half_h),
        ]
    }
}

impl Geometry for ColumnData {
    fn hit_test(&self, pos: Vector2, tolerance: f32) -> bool {
        // Simple hit test: transform point to local aligned space, check rect
        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;

        let cos_a = (-self.rotation).cos();
        let sin_a = (-self.rotation).sin();

        let local_x = dx * cos_a - dy * sin_a;
        let local_y = dx * sin_a + dy * cos_a;

        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        // Check if inside rect
        let inside = local_x.abs() <= half_w && local_y.abs() <= half_h;
        if inside {
            return true;
        }

        // Check edges (tolerance)
        let on_edge_x =
            (local_x.abs() - half_w).abs() < tolerance && local_y.abs() <= half_h + tolerance;
        let on_edge_y =
            (local_y.abs() - half_h).abs() < tolerance && local_x.abs() <= half_w + tolerance;

        on_edge_x || on_edge_y
    }

    fn center(&self) -> Vector2 {
        self.center
    }

    fn bounding_box(&self) -> (Vector2, Vector2) {
        let corners = self.get_corners();
        let mut min = Vector2::new(f32::MAX, f32::MAX);
        let mut max = Vector2::new(f32::MIN, f32::MIN);

        for p in corners {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }
        (min, max)
    }

    fn as_polyline(&self) -> Vec<Vector2> {
        let corners = self.get_corners();
        vec![corners[0], corners[1], corners[2], corners[3], corners[0]]
    }

    fn translate(&mut self, delta: Vector2) {
        self.center = self.center + delta;
    }

    fn rotate(&mut self, pivot: Vector2, angle: f32) {
        // Rotate center around pivot
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let dx = self.center.x - pivot.x;
        let dy = self.center.y - pivot.y;

        self.center = Vector2::new(
            pivot.x + dx * cos_a - dy * sin_a,
            pivot.y + dx * sin_a + dy * cos_a,
        );

        // Add rotation
        self.rotation += angle;
    }

    fn scale(&mut self, base: Vector2, factor: f32) {
        // Scale position
        let dx = self.center.x - base.x;
        let dy = self.center.y - base.y;
        self.center = Vector2::new(base.x + dx * factor, base.y + dy * factor);

        // Scale dimensions
        self.width *= factor;
        self.height *= factor;
    }

    fn is_closed(&self) -> bool {
        true
    }

    fn is_filled(&self) -> bool {
        // Columns are typically filled/hatched
        true
    }
}
