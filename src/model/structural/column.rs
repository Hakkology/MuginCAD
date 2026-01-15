use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Column instance - placed at a specific position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub type_id: String,          // Reference to ColumnType
    pub position: Vector2,        // Center position
    pub rotation: f32,            // Rotation in radians
    pub orientation: Orientation, // H/V toggle
    pub index: u32,               // Instance number (auto-assigned)
}

/// Column orientation - swaps width/depth visually
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal, // Width along X axis
    Vertical, // Width along Y axis (90° rotation)
}

/// Pivot point for move/rotate operations
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PivotPoint {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    #[default]
    Center,
}

impl Column {
    pub fn new(type_id: String, position: Vector2, index: u32) -> Self {
        Self {
            type_id,
            position,
            rotation: 0.0,
            orientation: Orientation::Horizontal,
            index,
        }
    }

    /// Get display name like "30x50-C1"
    pub fn display_name(&self) -> String {
        format!("{}-C{}", self.type_id, self.index)
    }

    /// Get center point
    pub fn center(&self) -> Vector2 {
        self.position
    }

    /// Hit test for column
    pub fn hit_test(&self, pos: Vector2, tolerance: f32, width: f32, depth: f32) -> bool {
        let (w, h) = match self.orientation {
            Orientation::Horizontal => (width, depth),
            Orientation::Vertical => (depth, width),
        };

        // For rotated rectangle hit test, we transform the test point to local coords
        let cos_r = (-self.rotation).cos();
        let sin_r = (-self.rotation).sin();
        let dx = pos.x - self.position.x;
        let dy = pos.y - self.position.y;
        let local_x = dx * cos_r - dy * sin_r;
        let local_y = dx * sin_r + dy * cos_r;

        let half_w = w / 2.0 + tolerance;
        let half_h = h / 2.0 + tolerance;

        local_x.abs() <= half_w && local_y.abs() <= half_h
    }

    /// Get the 4 corners of the column in world coordinates
    pub fn get_corners(&self, width: f32, depth: f32) -> [Vector2; 4] {
        let (w, h) = match self.orientation {
            Orientation::Horizontal => (width, depth),
            Orientation::Vertical => (depth, width),
        };

        let half_w = w / 2.0;
        let half_h = h / 2.0;

        let local_corners = [
            Vector2::new(-half_w, -half_h), // Bottom-left
            Vector2::new(half_w, -half_h),  // Bottom-right
            Vector2::new(half_w, half_h),   // Top-right
            Vector2::new(-half_w, half_h),  // Top-left
        ];

        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

        let mut world_corners = [Vector2::new(0.0, 0.0); 4];
        for (i, lc) in local_corners.iter().enumerate() {
            let rx = lc.x * cos_r - lc.y * sin_r + self.position.x;
            let ry = lc.x * sin_r + lc.y * cos_r + self.position.y;
            world_corners[i] = Vector2::new(rx, ry);
        }

        world_corners
    }

    /// Get 5 pivot points in world coordinates
    pub fn get_pivot_positions(&self, width: f32, depth: f32) -> [Vector2; 5] {
        let corners = self.get_corners(width, depth);
        [
            corners[3],    // Top-left
            corners[2],    // Top-right
            corners[0],    // Bottom-left
            corners[1],    // Bottom-right
            self.position, // Center
        ]
    }

    /// Toggle orientation (H <-> V)
    pub fn toggle_orientation(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal,
        };
    }
}
