use crate::model::Vector2;
use serde::{Deserialize, Serialize};

/// Axis orientation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AxisOrientation {
    /// Vertical axis (labeled A, B, C...)
    Vertical,
    /// Horizontal axis (labeled 1, 2, 3...)
    Horizontal,
}

/// Construction axis (infinite line for architectural grids)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Axis {
    /// Position on the perpendicular axis
    /// For Vertical: X position
    /// For Horizontal: Y position
    pub position: f32,
    /// Orientation of the axis
    pub orientation: AxisOrientation,
    /// Label (A, B, C... or 1, 2, 3...)
    pub label: String,
}

impl Axis {
    pub fn new(position: f32, orientation: AxisOrientation, label: String) -> Self {
        Self {
            position,
            orientation,
            label,
        }
    }

    /// Create a vertical axis at X position
    pub fn vertical(x_position: f32, label: String) -> Self {
        Self::new(x_position, AxisOrientation::Vertical, label)
    }

    /// Create a horizontal axis at Y position
    pub fn horizontal(y_position: f32, label: String) -> Self {
        Self::new(y_position, AxisOrientation::Horizontal, label)
    }

    /// Get start and end points for rendering (within viewport bounds)
    pub fn get_render_points(
        &self,
        viewport_min: Vector2,
        viewport_max: Vector2,
    ) -> (Vector2, Vector2) {
        match self.orientation {
            AxisOrientation::Vertical => {
                // Vertical line at X position
                (
                    Vector2::new(self.position, viewport_min.y),
                    Vector2::new(self.position, viewport_max.y),
                )
            }
            AxisOrientation::Horizontal => {
                // Horizontal line at Y position
                (
                    Vector2::new(viewport_min.x, self.position),
                    Vector2::new(viewport_max.x, self.position),
                )
            }
        }
    }

    /// Get label position (at the edge of viewport)
    pub fn get_label_position(&self, viewport_min: Vector2, viewport_max: Vector2) -> Vector2 {
        match self.orientation {
            AxisOrientation::Vertical => {
                // Label at top of vertical axis
                Vector2::new(self.position, viewport_max.y)
            }
            AxisOrientation::Horizontal => {
                // Label at left of horizontal axis
                Vector2::new(viewport_min.x, self.position)
            }
        }
    }
}

/// Axis manager - handles auto-labeling and axis collection
#[derive(Debug, Clone, Default)]
pub struct AxisManager {
    /// All axes
    pub axes: Vec<Axis>,
    /// Next vertical axis label (A, B, C...)
    next_vertical_index: usize,
    /// Next horizontal axis label (1, 2, 3...)
    next_horizontal_index: usize,
}

impl AxisManager {
    pub fn new() -> Self {
        Self {
            axes: Vec::new(),
            next_vertical_index: 0,
            next_horizontal_index: 1,
        }
    }

    /// Get next auto-label for vertical axis (A, B, C, ..., Z, AA, AB...)
    fn next_vertical_label(&mut self) -> String {
        let label = Self::index_to_letter(self.next_vertical_index);
        self.next_vertical_index += 1;
        label
    }

    /// Get next auto-label for horizontal axis (1, 2, 3...)
    fn next_horizontal_label(&mut self) -> String {
        let label = self.next_horizontal_index.to_string();
        self.next_horizontal_index += 1;
        label
    }

    /// Convert index to letter (0=A, 1=B, ..., 25=Z, 26=AA, 27=AB...)
    fn index_to_letter(index: usize) -> String {
        if index < 26 {
            char::from_u32('A' as u32 + index as u32)
                .unwrap()
                .to_string()
        } else {
            let first = (index / 26) - 1;
            let second = index % 26;
            format!(
                "{}{}",
                char::from_u32('A' as u32 + first as u32).unwrap(),
                char::from_u32('A' as u32 + second as u32).unwrap()
            )
        }
    }

    /// Add a vertical axis at X position with auto-label
    pub fn add_vertical(&mut self, x_position: f32) -> &Axis {
        let label = self.next_vertical_label();
        self.axes.push(Axis::vertical(x_position, label));
        self.axes.last().unwrap()
    }

    /// Add a horizontal axis at Y position with auto-label
    pub fn add_horizontal(&mut self, y_position: f32) -> &Axis {
        let label = self.next_horizontal_label();
        self.axes.push(Axis::horizontal(y_position, label));
        self.axes.last().unwrap()
    }

    /// Clear all axes
    pub fn clear(&mut self) {
        self.axes.clear();
        self.next_vertical_index = 0;
        self.next_horizontal_index = 1;
    }

    /// Remove axis by index
    pub fn remove(&mut self, index: usize) {
        if index < self.axes.len() {
            self.axes.remove(index);
        }
    }
}
